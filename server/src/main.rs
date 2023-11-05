use args::*;
use rocket::fs::NamedFile;
use rocket::{catchers, get, launch, routes, State};
use rocket_ws::{Channel, Message, WebSocket};
use std::path::Path;

mod args;
mod catchers;
mod debugging;

#[launch]
fn rocket() -> _ {
    println!("{:?}", std::env::current_dir());
    println!("{:?}", std::env::args().collect::<String>());

    let (sender, receiver) = crossbeam::channel::unbounded::<Message>();
    let args = Args::read_env();
    let mut server = rocket::build()
        .mount("/", routes![index, favicon, js, wasm, receive_sound, send_sound])
        .manage(Sockets {sender,receiver})
        .register("/", catchers![catchers::not_found]);

    server = debugging::attach(server, &args);

    server
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    println!("{:?}", std::env::current_dir());
    NamedFile::open(Path::new("./static/index.html")).await.ok()
}

#[get("/receive-sound")] // receive - for client
async fn receive_sound(ws: WebSocket, sockets: &State<Sockets>) -> Channel<'static> {
    println!("Socket received");
    use rocket::futures::{SinkExt};

    let receiver = sockets.receiver.clone();
    ws.channel(move |mut stream| Box::pin(async move {
        for message in receiver.iter() {
            let _ = stream.send(message).await;
        }

        Ok(())
    }))
}

#[get("/send-sound")] // send - for client
async fn send_sound(ws: WebSocket, sockets: &State<Sockets>) -> Channel<'static> {
    println!("Socket received");
    use rocket::futures::{StreamExt};

    let sender = sockets.sender.clone();
    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            let _ = sender.send(message?);
        }

        Ok(())
    }))
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
    pub sender: crossbeam::channel::Sender<Message>,
    pub receiver: crossbeam::channel::Receiver<Message>,
}
