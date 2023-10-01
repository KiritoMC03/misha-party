use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::{catch, catchers, Data, get, launch, Request, routes};
use rocket::fairing::{Fairing, Info, Kind};
use rocket_ws::{Stream, WebSocket};

#[launch]
fn rocket() -> _ {
    let res = rocket::build()
        .mount("/", routes![index, favicon, echo_stream, log_req])
        .register("/", catchers![not_found])
        .attach(Test{});
    for route in res.routes() {
        println!("{}", route);
    }

    res
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/<p>")]
async fn log_req(p: PathBuf) {
    println!("{}", p.as_path().to_str().unwrap());
}

#[get("/echo")]
async fn echo_stream(ws: WebSocket) -> Stream!['static] {
    println!("ws enter 1");
    Stream! { ws =>
        for await message in ws {
            let message = message.unwrap();
            println!("{}", message);
            yield message;
        }
    }
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).await.ok()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

struct Test;

#[rocket::async_trait]
impl Fairing for Test {
    fn info(&self) -> Info {
        Info {
            name: "Test",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        println!("req for test:_ {}", req);

        use tungstenite::handshake::derive_accept_key;
        use rocket::http::uncased::eq;

        let headers = req.headers();
        let is_upgrade = headers.get("Connection")
            .any(|h| h.split(',').any(|v| eq(v.trim(), "upgrade")));

        let is_ws = headers.get("Upgrade")
            .any(|h| h.split(',').any(|v| eq(v.trim(), "websocket")));

        let is_13 = headers.get_one("Sec-WebSocket-Version").map_or(false, |v| v == "13");
        let key = headers.get_one("Sec-WebSocket-Key").map(|k| derive_accept_key(k.as_bytes()));
        match key {
            Some(key) if is_upgrade && is_ws && is_13 => {
                println!("key success: {key}");
            },
            Some(_) | None => {
                println!("key failed: {} - {} - {}", is_upgrade, is_ws, is_13);
            }
        };
    }
}