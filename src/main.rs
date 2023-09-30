
use std::path::Path;
use rocket::{get, launch, routes};
use rocket::fs::NamedFile;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}

#[get("/")]
async fn main_page() -> Option<NamedFile> {
    println!("GET HERE");
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/")]
fn index(a: rocket_client_addr::ClientAddr, b: rocket_client_addr::ClientRealAddr) -> String {
    format!("Remote Address: {:?}, {:?}", a, b)
}