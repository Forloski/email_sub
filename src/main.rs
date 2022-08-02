use email_sub::{configuration::get_configuration, db_connection, startup::run};
use std::{net::TcpListener, sync::Arc};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_conn = db_connection::connection()
        .await
        .expect("Failed to connect to EdgeDB.");
    let configuration = Arc::new(get_configuration().expect("Failed to load configuration."));
    let address = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, db_conn.clone())?.await
}
