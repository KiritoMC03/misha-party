use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::{catch, catchers, Data, get, launch, Request, routes};
use rocket::fairing::{Fairing, Info, Kind};
use rocket_ws::{Stream, WebSocket};

#[launch]
fn rocket() -> _ {
    let res = rocket::build()
        .mount("/", routes![index, favicon, echo_stream, log_req])
        .register("/", catchers![not_found])
        .attach(Test{});
    for route in res.routes() {
        println!("{}", route);
    }

    res
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/<p>")]
async fn log_req(p: PathBuf) {
    println!("{}", p.as_path().to_str().unwrap());
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

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).await.ok()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

struct Test;

impl Fairing for Test {
    fn info(&self) -> Info {
        Info {
            name: "Test",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, _req: &mut Request<'_>, _data: &mut Data<'_>) {
        println!("req for test:_ {}", _req);
    }
}