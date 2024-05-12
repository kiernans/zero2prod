use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health-check", web::get().to(health_check)))
        .bind("localhost:8000")?
        .run();

    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
