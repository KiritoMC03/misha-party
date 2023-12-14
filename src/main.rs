use std::path::Path;
use rocket::fs::NamedFile;
use rocket::{catchers, get, launch, routes};
use rocket_ws::{Stream, WebSocket};

mod debugging;
mod catchers;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, favicon, echo_stream])
        .register("/", catchers![catchers::not_found])
        .attach(debugging::RequestLogger)
        .attach(debugging::WebSocketConnectionLogger)
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/echo")]
async fn echo_stream(ws: WebSocket) -> Stream!['static] {
    Stream! { ws =>
        for await message in ws {
            let message = message.unwrap();
            println!("WebSocket message: \"{}\"", message);
            yield message;
        }
    }
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).await.ok()
}