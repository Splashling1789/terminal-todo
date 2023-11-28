
use std::io::stdin;
use clearscreen::clear;
use sqlx::SqliteConnection;
use crate::db_manager::{print_todos, get_todos, delete_task, insert_task, alter_done};
pub async fn launch_menu(conn: &mut SqliteConnection, show_done: &mut bool) {
        clear().expect("Failed to clear screen");
        let list = get_todos(conn).await.expect("Failed to obtain db content from table tasks");
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
                alter_done(conn, id).await;
            }
            Err(_) => {
                if content.starts_with("/") {
                    match content {
                        "/done" => { *show_done = !*show_done; }
                        "/delete" => {
                            input = String::default();
                            println!("Input the id or description of the task you want to delete: ");
                            stdin().read_line(&mut input).expect("Failed to get stdin");
                            delete_task(conn, input.trim()).await;
                        }
                        _ => {}
                    }
                }
                else {
                    insert_task(conn, content).await;
                }
            }
        }
    }