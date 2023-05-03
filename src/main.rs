use actix_web::{HttpServer, App, web, middleware::{Logger, self}, http::header};
use dotenv::dotenv;
use std::env;
use env_logger::Env;
use actix_cors::Cors;
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello Word");

    if std::env::var_os("APP_LOG").is_none() {
        std::env::set_var("APP_LOG", "actix_web=info");
    }
    dotenv().expect("Failed to read .env file");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .supports_credentials();
        
        App::new()
        .wrap(middleware::Compress::default())
        .wrap(cors)
        .configure(handler::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}