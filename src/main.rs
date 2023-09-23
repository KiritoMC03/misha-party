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
const PORT : u32 = 5000;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = App::new()
            .service(main_page);
    HttpServer::new(|| {app})
        .bind((IP, PORT))?
        .run()
        .await
}

#[get("/")]
async fn main_page() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
