use std::path::Path;
use rocket::fs::NamedFile;
use rocket::{catch, catchers, get, launch, Request, routes};
use rocket_ws::{Stream, WebSocket};

#[launch]
fn rocket() -> _ {
    let res = rocket::build()
        .mount("/", routes![index, favicon, echo_stream, echo_text])
        .register("/", catchers![not_found]);
    for route in res.routes() {
        println!("{}", route);
    }

    res
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
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

#[get("/echo")]
async fn echo_text() -> String {
    "huiyak".to_string()
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).await.ok()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}