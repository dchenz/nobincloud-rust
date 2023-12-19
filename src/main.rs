use server::ServerConfig;

mod model;
mod mysql;
mod server;
mod services;

fn get_config() -> ServerConfig {
    let hostname = std::env::var("SERVER_HOST").unwrap_or("localhost".to_owned());

    let port = std::env::var("SERVER_PORT")
        .unwrap_or("5000".to_owned())
        .parse()
        .unwrap_or_else(|e| {
            println!("Unable to parse SERVER_PORT: {}", e);
            std::process::exit(1);
        });

    let db_connection_string = std::env::var("DATABASE_URL").unwrap_or_else(|e| {
        println!("Unable to read DATABASE_URL: {}", e);
        std::process::exit(1);
    });

    ServerConfig {
        hostname,
        port,
        db_connection_string,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    server::run(get_config()).await;
}
