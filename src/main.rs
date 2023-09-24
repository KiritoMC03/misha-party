#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![default_page, hello])
}

#[get("/")]
fn default_page() -> &'static str {
    "Misha - pidor!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}