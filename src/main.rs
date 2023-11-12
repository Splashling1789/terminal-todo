use sqlx::{Connection, Error, query, Row, SqliteConnection};
use sqlx::sqlite::SqliteQueryResult;
use std::io::stdin;
use clearscreen::clear;

const SQLITE_FILENAME:&str = "db.sqlite";
const QUERY_INITIALIZE_TABLES:&str = "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, description TEXT NOT NULL, done INTEGER DEFAULT 0);";
const QUERY_GET_TODOS:&str = "SELECT * FROM tasks";
const QUERY_ALTER_TODO:&str = "UPDATE tasks SET done = CASE WHEN done = 0 THEN 1 ELSE 0 END WHERE id = ?;";
const QUERY_DELETE_WHERE_ID : &str = "DELETE FROM tasks WHERE id= ?";
const QUERY_DELETE_WHERE_DESC : &str ="DELETE FROM tasks WHERE description= ?;";
const QUERY_INSERT_VALUE: &str = "INSERT INTO tasks (description) VALUES (?);";
#[derive(Debug)]
struct ToDo {
    id: i32,
    description: String,
    done: bool
}

#[tokio::main]
async fn main() {
    println!("Welcome to the Terminal-ToDo!");
    println!("Connecting to {SQLITE_FILENAME}");
    rusqlite::Connection::open(SQLITE_FILENAME).expect("Failed to create database");
    let mut conn = manage_connection().await.expect("An error ocurred connecting to the database");
    match initialize_db(&mut conn).await {
        Ok(_) => {
            println!("Connected correctly!");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
    let mut show_done = false;
    loop {
        clear().expect("Failed to clear screen");
        let list = get_todos(&mut conn).await.expect("Failed to obtain db content from table tasks");
        println!("Terminal-Todo\n");
        print_todos(list, show_done);
        println!("- Type a description to create a new task.");
        println!("- Type the task id you want to mark as done or undone.");
        println!("- Type \"/done\" to alternate showing or not showing done tasks.");
        println!("- Type \"/delete\" to delete a task with a given id.");
        let mut input = String::default();

        stdin().read_line(&mut input).expect("Failed to get stdin");
        let content= input.trim();
        match input.trim().parse::<i32>() {
            Ok(id) => {
                query(QUERY_ALTER_TODO).bind(id).execute(&mut conn).await.expect("Error executing the query");
            }
            Err(_) => {
                match content {
                    "/done" => {show_done = !show_done;}
                    "/delete" => {
                        input = String::default();
                        println!("Input the id or description of the task you want to delete: ");
                        stdin().read_line(&mut input).expect("Failed to get stdin");
                        match input.trim().parse::<i32>() {
                            Ok(id) => {
                                query(QUERY_DELETE_WHERE_ID).bind(id).execute(&mut conn).await.expect("Failed to execute query");
                            }
                            Err(_) => {
                                query(QUERY_DELETE_WHERE_DESC).bind(input.trim()).execute(&mut conn).await.expect("Failed to execute query");
                            }
                        }
                    }
                    content => {
                        query(QUERY_INSERT_VALUE).bind(content).execute(&mut conn).await.expect("Failed to execute query");
                    }
                }
            }
        }
    }

}

async fn manage_connection() -> Option<SqliteConnection> {
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

async fn get_todos(conn: &mut SqliteConnection) -> Option<Vec<ToDo>> {
    match query(QUERY_GET_TODOS).fetch_all(conn).await {
        Ok(q) => {
            let mut list: Vec<ToDo> = vec![];
            for r in q {
                list.push(ToDo {
                    id: r.get("id"),
                    description: r.get("description"),
                    done: match r.get("done") {
                        0 => false,
                        _ => true
                    }
                })
            }
            Some(list)
        }
        Err(e) => {
            println!("Ocurrió un error con la consulta: {e}");
            None
        }
    }
}

fn print_todos(list: Vec<ToDo>, print_done: bool) {
    if !print_done {
        for t in list {
            if !t.done {
                println!("[O#{}] {}", t.id, t.description);
            }
        }
    }
    else {
        for t in list {
            let x = match t.done {
                false => "O",
                true => "X"
            };
            println!("[{}#{}] {}", x, t.id, t.description);
        }
    }
}