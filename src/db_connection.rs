use edgedb_tokio::{Client, Error};

pub async fn connection() -> Result<Client, Error> {
    let conn = edgedb_tokio::create_client().await?;

    Ok(conn)
}
