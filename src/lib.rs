use actix_web::{dev::Server, get, App, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    const PORT: u16 = 8081;

    let server = HttpServer::new(|| App::new().service(health_check))
        .bind(("localhost", PORT))?
        .run();

    Ok(server)
}
