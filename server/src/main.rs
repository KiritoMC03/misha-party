use std::path::Path;
use rocket::fs::NamedFile;
use rocket::{catchers, get, launch, routes, State};
use rocket::tokio::sync::Mutex;
use rocket_ws::WebSocket;
use args::*;

mod debugging;
mod catchers;
mod args;

#[launch]
fn rocket() -> _ {
    let args = Args::read_env();
    let mut server = rocket::build()
        .mount("/", routes![index, favicon, echo_stream])
        .manage(Sockets{ list: Mutex::new(Vec::new()) })
        .register("/", catchers![catchers::not_found]);

    server = debugging::attach(server, &args);

    server
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../../static/index.html")).await.ok()
}

#[get("/echo")]
async fn echo_stream(_ws: WebSocket, _sockets: &State<Sockets>) {
    // sockets.list.lock().await.push(ws);

    // ws.stream(|io| {
    //
    // })
    // Stream! { ws =>
    //     for await message in ws {
    //         let message = message.unwrap();
    //         println!("WebSocket message: \"{}\"", message);
    //         yield message;
    //     }
    // }
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("../../static/favicon.ico")).await.ok()
}

struct Sockets {
    pub list: Mutex<Vec<WebSocket>>,
}