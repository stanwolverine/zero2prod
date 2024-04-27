use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello, {name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 8081;

    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("localhost", PORT))?
        .run()
        .await
}
