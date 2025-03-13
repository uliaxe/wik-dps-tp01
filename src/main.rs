mod handlers;
mod config;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = config::get_server_port();
    
    println!("🚀 Serveur en écoute sur le port {}", port);

    HttpServer::new(|| {
        App::new()
            .service(handlers::ping) // Route /ping
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
