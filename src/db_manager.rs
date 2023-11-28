use sqlx::{query, Row, SqliteConnection};

const QUERY_GET_TODOS:&str = "SELECT * FROM tasks";
const QUERY_DELETE_WHERE_ID : &str = "DELETE FROM tasks WHERE id= ?";
const QUERY_DELETE_WHERE_DESC : &str ="DELETE FROM tasks WHERE description= ?;";
const QUERY_INSERT_VALUE: &str = "INSERT INTO tasks (description) VALUES (?);";
const QUERY_ALTER_TODO:&str = "UPDATE tasks SET done = CASE WHEN done = 0 THEN 1 ELSE 0 END WHERE id = ?;";
#[derive(Debug)]
pub struct ToDo {
    id: i32,
    description: String,
    done: bool
}

pub async fn delete_task(conn: &mut SqliteConnection, t: &str) {
    match t.parse::<i32>() {
        Ok(id) => {
            query(QUERY_DELETE_WHERE_ID).bind(id).execute(conn).await.expect("Failed to execute query");
        }
        Err(_) => {
            query(QUERY_DELETE_WHERE_DESC).bind(t).execute(conn).await.expect("Failed to execute query");
        }
    }
}

pub async fn insert_task(conn: &mut SqliteConnection, content: &str){
    query(QUERY_INSERT_VALUE).bind(content).execute(conn).await.expect("Failed to execute query");
}

pub async fn alter_done(conn: &mut SqliteConnection, id: i32) {
    query(QUERY_ALTER_TODO).bind(id).execute(conn).await.expect("Failed to execute query");
}

pub async fn get_todos(conn: &mut SqliteConnection) -> Option<Vec<ToDo>> {
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

pub fn print_todos(list: Vec<ToDo>, print_done: &mut bool) {
    if !*print_done {
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