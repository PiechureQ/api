use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use std::env;
use dotenv::dotenv;
use log::{info};
use routes::{places as places_routes};

mod error;
mod db;
mod routes;
mod model;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
                        App::new().configure(places_routes::routes)
                    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await
}
