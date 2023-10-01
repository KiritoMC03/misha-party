use std::env;
use std::path::Path;
use rocket::fs::NamedFile;
use rocket::{catchers, get, launch, routes};
use rocket_ws::{Stream, WebSocket};
use args::*;

mod debugging;
mod catchers;
mod args;

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = env::args().collect();
    let mut server = rocket::build()
        .mount("/", routes![index, favicon, echo_stream])
        .register("/", catchers![catchers::not_found]);

    if args.contains(&LOG_REQUESTS.to_string()) {
        server = server.attach(debugging::RequestLogger);
    }
    if args.contains(&LOG_WS_CONNECTIONS.to_string()) {
        server = server.attach(debugging::WebSocketConnectionLogger)
    }

    server
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