use std::path::Path;
use rocket::fs::NamedFile;
use rocket::{get, launch, routes};


use rocket_ws::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![main_page, echo_stream])
}

#[get("/")]
async fn main_page() -> Option<NamedFile> {
    println!("GET HERE");
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/ws")]
fn echo_stream(ws: WebSocket) -> Stream!['static] {
    Stream! { ws =>
        for await message in ws {
            yield message?;
        }
    }
}