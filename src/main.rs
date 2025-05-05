use std::env;
use std::process;
use tokio_postgres::{Error, NoTls};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key_user = "DB_USER";
    let user = match env::var(key_user) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, key_user);
            process::exit(1);
        }
    };

    let key_password = "DB_PASSWORD";
    let password = match env::var(key_password) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, key_password);
            process::exit(1);
        }
    };

    let key_host = "DB_HOST";
    let host = match env::var(key_host) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, key_host);
            process::exit(1);
        }
    };

    let key_port = "DB_PORT";
    let port = match env::var(key_port) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, key_port);
            process::exit(1);
        }
    };

    let key_database = "DB_DATABASE";
    let database = match env::var(key_database) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, key_database);
            process::exit(1);
        }
    };

    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, database
        ),
        NoTls,
    )
    .await?;

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
