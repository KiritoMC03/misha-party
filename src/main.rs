use actix_web::{
    App,
    HttpResponse,
    HttpServer,
    Responder
};

use actix_web:: {
    get,
};

const IP : &str = "0.0.0.0";
const PORT : &str = "5000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| PORT.to_string())
        .parse()
        .expect("PORT must be a number");

    let app = App::new()
            .service(main_page);
    HttpServer::new(|| {app})
        .bind((IP, port))?
        .run()
        .await
}

#[get("/")]
async fn main_page() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
