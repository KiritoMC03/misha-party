use std::io::ErrorKind;
use std::net::UdpSocket;
use std::path::Path;
use std::process;
use std::thread;
use std::time::Instant;
use rocket::fs::NamedFile;
use rocket::{get, launch, post, routes};

use str0m::change::SdpOffer;
use str0m::net::Receive;
use str0m::IceConnectionState;
use str0m::{Candidate, Event, Input, Output, Rtc, RtcError};

use std::net::IpAddr;
use rocket::log::private::__private_api::log;
use rocket::serde::json::Json;
use systemstat::{Platform, System};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![main_page, web_request])
}

#[get("/")]
async fn main_page() -> Option<NamedFile> {
    println!("GET HERE");
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[post("/", format = "application/json", data = "<offer>")]
fn web_request(offer: Json<SdpOffer>) -> String {
    println!("POST HERE");
    let mut rtc = Rtc::new();

    let addr = select_host_address();

    // Spin up a UDP socket for the RTC
    let socket = UdpSocket::bind(format!("{addr}:0")).expect("binding a random UDP port");
    let addr = socket.local_addr().expect("a local socket adddress");
    let candidate = Candidate::host(addr).expect("a host candidate");
    rtc.add_local_candidate(candidate);

    // Create an SDP Answer.
    let answer = rtc
        .sdp_api()
        .accept_offer(offer.0)
        .expect("offer to be accepted");

    // Launch WebRTC in separate thread.
    thread::spawn(|| {
        if let Err(e) = run(rtc, socket) {
            eprintln!("Exited: {e:?}");
            process::exit(1);
        }
    });

    let body = serde_json::to_string(&answer).expect("answer to serialize");
    body
}

fn run(mut rtc: Rtc, socket: UdpSocket) -> Result<(), RtcError> {
    // Buffer for incoming data.
    let mut buf = Vec::new();

    loop {
        // Poll output until we get a timeout. The timeout means we are either awaiting UDP socket input
        // or the timeout to happen.
        let timeout = match rtc.poll_output()? {
            Output::Timeout(v) => v,

            Output::Transmit(v) => {
                println!("content: {:?}, {:?}", &v.contents, v.destination);
                socket.send_to(&v.contents, v.destination)?;
                continue;
            }

            Output::Event(v) => {
                if v == Event::IceConnectionStateChange(IceConnectionState::Disconnected) {
                    println!("disconnect");
                    return Ok(());
                }
                continue;
            }
        };

        let timeout = timeout - Instant::now();

        // socket.set_read_timeout(Some(0)) is not ok
        if timeout.is_zero() {
            rtc.handle_input(Input::Timeout(Instant::now()))?;
            continue;
        }

        socket.set_read_timeout(Some(timeout))?;
        buf.resize(2000, 0);

        let input = match socket.recv_from(&mut buf) {
            Ok((n, source)) => {
                buf.truncate(n);
                Input::Receive(
                    Instant::now(),
                    Receive {
                        source,
                        destination: socket.local_addr().unwrap(),
                        contents: buf.as_slice().try_into()?,
                    },
                )
            }

            Err(e) => match e.kind() {
                // Expected error for set_read_timeout(). One for windows, one for the rest.
                ErrorKind::WouldBlock | ErrorKind::TimedOut => Input::Timeout(Instant::now()),
                _ => return Err(e.into()),
            },
        };

        println!("input: {:?}", input);
        rtc.handle_input(input)?;
    }
}

pub fn select_host_address() -> IpAddr {
    let system = System::new();
    let networks = system.networks().unwrap();

    for net in networks.values() {
        for n in &net.addrs {
            if let systemstat::IpAddr::V4(v) = n.addr {
                if !v.is_loopback() && !v.is_link_local() && !v.is_broadcast() {
                    return IpAddr::V4(v);
                }
            }
        }
    }

    panic!("Found no usable network interface");
}