use args::*;
use rocket::fs::NamedFile;
use rocket::{catchers, get, launch, routes, State, tokio};
use rocket_ws::{Channel, Message, WebSocket};
use std::path::Path;
use rocket::tokio::sync::broadcast;

mod args;
mod catchers;
mod debugging;

static mut INDEX_INCOME: u32 = 0u32;
static mut INDEX_OUTCOME: u32 = 0u32;

#[launch]
fn rocket() -> _ {
    println!("{:?}", std::env::current_dir());
    println!("{:?}", std::env::args().collect::<String>());

    let (sender, receiver) = broadcast::channel::<Message>(16);
    let args = Args::read_env();
    let mut server = rocket::build()
        .mount("/", routes![index, favicon, js, wasm, client_income_socket, client_outcome_socket])
        .manage(Sockets {sender,receiver})
        .register("/", catchers![catchers::not_found]);

    server = debugging::attach(server, &args);

    server
}

#[get("/income")] // income - for client
async fn client_income_socket(ws: WebSocket, sockets: &State<Sockets>) -> Channel<'static> {
    println!("income");
    use rocket::futures::{SinkExt};

    let mut receiver = sockets.sender.subscribe();
    ws.channel(move |mut stream| Box::pin(async move {
        tokio::spawn(async move {
            let idx: u32;
            unsafe {
                idx = INDEX_INCOME;
                INDEX_INCOME += 1;
            }
            while let Ok(message) = receiver.recv().await {
                println!("--- income {} before --- {}", message.to_string(), idx);
                let _ = stream.send(message).await;
                println!("--- income after --- {}", idx);
            }
            println!("--- income EXIT --- {}", idx);
        });

        Ok(())
    }))
}

#[get("/outcome")] // outcome - for client
async fn client_outcome_socket(ws: WebSocket, sockets: &State<Sockets>) -> Channel<'static> {
    println!("outcome");
    use rocket::futures::{StreamExt};

    let sender = sockets.sender.clone();
    ws.channel(move |mut stream| Box::pin(async move {
        tokio::spawn(async move {
            let idx: u32;
            unsafe {
                idx = INDEX_OUTCOME;
                INDEX_OUTCOME += 1;
            }
            while let Some(message) = stream.next().await {
                let unwrapped = message.unwrap();
                println!("--- outcome {} before --- {}", unwrapped.to_string(), idx);
                let _ = sender.send(unwrapped);
                println!("--- outcome after --- {}", idx);
            }

            println!("--- outcome EXIT --- {}", idx);
        });

        Ok(())
    }))
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    println!("{:?}", std::env::current_dir());
    NamedFile::open(Path::new("./static_gen/index.html")).await.ok()
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/favicon.ico"))
        .await
        .ok()
}

#[get("/yew.js")]
async fn js() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static_gen/yew.js"))
        .await
        .ok()
}

#[get("/yew.wasm")]
async fn wasm() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static_gen/yew.wasm"))
        .await
        .ok()
}

struct Sockets {
    pub sender: broadcast::Sender<Message>,
    receiver: broadcast::Receiver<Message>,
}
