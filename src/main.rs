use tokio_postgres::{Error, NoTls};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TODO: Environment variables for database connection

    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=pass", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query(
            "SELECT id, title, content, done, version, created_at, updated_at FROM todo",
            &[],
        )
        .await?;

    // Iterate over the rows returned by the query.
    rows.iter().for_each(|row| {
        let id: Uuid = row.get(0);
        let title: &str = row.get(1);
        let content: &str = row.get(2);
        let done: bool = row.get(3);
        let version: i32 = row.get(4);
        let created_at: chrono::NaiveDateTime = row.get(5);
        let updated_at: chrono::NaiveDateTime = row.get(6);

        println!(
            "id: {}, title: {}, content: {}, done: {}, version: {}, created_at: {}, updated_at: {}",
            id, title, content, done, version, created_at, updated_at
        );
    });

    Ok(())
}
