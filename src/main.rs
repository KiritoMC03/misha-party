use std::path::Path;
use rocket::fs::NamedFile;
use rocket::{get, launch, routes};
use rocket_ws::{Stream, WebSocket};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![main_page, echo_stream, favicon, echo_stream_s, echo_stream_ss, echo_stream_sss])
}

#[get("/")]
async fn main_page() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/ws")]
fn echo_stream(ws: WebSocket) -> Stream!['static] {
    println!("ws enter 1");
    Stream! { ws =>
        for await message in ws {
            let message = message.unwrap();
            println!("{}", message);
            yield message;
        }
    }
}

#[get("/ws/ws")]
fn echo_stream_s(ws: WebSocket) -> Stream!['static] {
    println!("ws enter 2");
    Stream! { ws =>
        for await message in ws {
            let message = message.unwrap();
            println!("{}", message);
            yield message;
        }
    }
}

#[get("/wss")]
fn echo_stream_ss(ws: WebSocket) -> Stream!['static] {
    println!("ws enter 3");
    Stream! { ws =>
        for await message in ws {
            let message = message.unwrap();
            println!("{}", message);
            yield message;
        }
    }
}

#[get("/wss/ws")]
fn echo_stream_sss(ws: WebSocket) -> Stream!['static] {
    println!("ws enter 4");
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