use crate::server::routes::health;

use actix_web::{App, HttpServer};
use anyhow::Result;


pub mod routes;

pub async fn start_server(host: &str, port: u32)
-> Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .service(health::health)
    });

    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
