mod connection_manager;
mod menu;
mod db_manager;
use crate::connection_manager::create_connection;
use crate::menu::launch_menu;


#[tokio::main]
async fn main() {
    println!("Welcome to the Terminal-ToDo!");
    match create_connection().await {
        Some(mut conn) => {
            let mut show_done = false;
            loop {
                launch_menu(&mut conn, &mut show_done).await;
            }
        }
        None => {
            println!("Failed to create the database connection.");
        }
    }
}



