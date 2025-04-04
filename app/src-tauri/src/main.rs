// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::auth::{login , register ,  key_gen};
use actix_web::{App, HttpRequest, HttpServer, middleware, web};
use env_logger;
use log;
use local_ip_address::local_ip;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    key_gen();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let device_ip = local_ip().unwrap();
    let bind_address = format!("{}:80", device_ip);
    log::info!("Starting at http://{}", bind_address);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(login)
            .service(register)
            .service(web::resource("/index.html").to(|| async { "Working?" })) // Remove when testing done
    }).bind(bind_address)?
    .run()
    .await
}
