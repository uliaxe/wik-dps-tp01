use std::env;
use dotenv::dotenv;

pub fn get_server_port() -> u16 {
    dotenv().ok();

    env::var("PING_LISTEN_PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .expect("PORT must be a valid number")
}