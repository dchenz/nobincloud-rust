mod model;
mod mysql;
mod server;
mod services;

use tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let connection_string = match std::env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(e) => {
            println!("Unable to read DATABASE_URL: {}", e.to_string());
            std::process::exit(1);
        }
    };
    server::run(&connection_string).await;
}
