use sqlx::{Connection, SqliteConnection};

const SQLITE_FILENAME:&str = "db.sqlite";
const QUERY_INITIALIZE_TABLES:&str = "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, description TEXT NOT NULL, done INTEGER DEFAULT 0);";

fn main() {
    println!("Welcome to the Terminal-ToDo!");
    println!("Connecting to {SQLITE_FILENAME}");
    let conn = manage_connection().expect("An error ocurred connecting to the database");

}

async fn manage_connection() -> Option<SqliteConnection> {
    match SqliteConnection::connect(SQLITE_FILENAME).await {
        Ok(conn) => {
            Some(conn)
        }
        Err(e) => {
            None
        }
    }
}

async fn initialize_db(conn: &SqliteConnection) {
    query("")
}
