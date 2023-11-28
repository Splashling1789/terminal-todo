use sqlx::{Connection, Error, query, SqliteConnection};
use sqlx::sqlite::SqliteQueryResult;

const SQLITE_FILENAME:&str = "db.sqlite";
const QUERY_INITIALIZE_TABLES:&str = "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, description TEXT NOT NULL, done INTEGER DEFAULT 0);";

async fn manage_connection() -> Option<SqliteConnection> {
    println!("Connecting to {SQLITE_FILENAME}");
    match SqliteConnection::connect(SQLITE_FILENAME).await {
        Ok(conn) => {
            Some(conn)
        }
        Err(_) => {
            None
        }
    }
}

async fn initialize_db(conn: &mut SqliteConnection) -> Result<SqliteQueryResult, Error> {
    query(QUERY_INITIALIZE_TABLES).execute(conn).await
}

async fn db_connect() -> SqliteConnection {
    rusqlite::Connection::open(SQLITE_FILENAME).expect("Failed to create database");
    manage_connection().await.expect("An error ocurred connecting to the database")
}

pub async fn create_connection() -> Option<SqliteConnection>{
    let mut conn = db_connect().await;
    match initialize_db(&mut conn).await {
        Ok(_) => {
            println!("Connected correctly!");
            Some(conn)
        }
        Err(e) => {
            println!("Error: {e}");
            None
        }
    }
}

