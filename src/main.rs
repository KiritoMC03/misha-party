use std::path::Path;
use rocket::fs::NamedFile;
use rocket::{get, launch, routes};
use rocket_ws::{Stream, WebSocket};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, favicon])
        .mount("/", routes![echo_stream])
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/echo")]
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

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).await.ok()
}