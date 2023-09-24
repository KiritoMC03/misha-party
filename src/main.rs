#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![default_page])
}

#[get("/")]
fn default_page() -> &'static str {
    "Misha - pidor!"
}