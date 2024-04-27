extern crate zero2prod;
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    zero2prod::run().unwrap().await
}
