use args::*;
use rocket::fs::NamedFile;
use rocket::tokio::sync::Mutex;
use rocket::{catchers, get, launch, routes};
use rocket_ws::{Stream, WebSocket};
use std::path::Path;

mod args;
mod catchers;
mod debugging;

#[launch]
fn rocket() -> _ {
    println!("{:?}", std::env::current_dir());
    println!("{:?}", std::env::args().collect::<String>());

    let args = Args::read_env();
    let mut server = rocket::build()
        .mount("/", routes![index, favicon, js, wasm, echo_stream])
        .manage(Sockets {
            _list: Mutex::new(Vec::new()),
        })
        .register("/", catchers![catchers::not_found]);

    server = debugging::attach(server, &args);

    server
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    println!("{:?}", std::env::current_dir());
    NamedFile::open(Path::new("./static/index.html")).await.ok()
}

#[get("/echo")]
async fn echo_stream(ws: WebSocket) -> Stream!['static] {
    println!("Socket received");
    // sockets.list.lock().await.push(ws);

    ws.stream(|io| {
        io
    })
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
    NamedFile::open(Path::new("./static/favicon.ico"))
        .await
        .ok()
}

#[get("/yew.js")]
async fn js() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/yew.js"))
        .await
        .ok()
}

#[get("/yew.wasm")]
async fn wasm() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/yew.wasm"))
        .await
        .ok()
}

struct Sockets {
    pub _list: Mutex<Vec<WebSocket>>,
}
