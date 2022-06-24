#![warn(clippy::all)]

use rusqlite::{Connection, Result};
use std::env;

mod todo;

use crate::todo::Task;

static DEBUG: bool = true;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    if DEBUG {
        eprintln!("argc: {}", args.len());
        dbg!(&args);
    }

    let db_path: &str = "/users/koury/dropbox/todo/todo.db";
    let connection = Connection::open(&db_path).unwrap();

    if args.len() == 1 {
        let mut stmt = connection.prepare("SELECT id, title FROM Todo")?;
        let task_iter = stmt.query_map([], |row| {
            eprintln!("found");
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })?;

        for task in task_iter {
            println!("{:?}", task.unwrap());
        }
    } else if args[1] == "init" {
        eprintln!("TODO: prompt the user before doing this. also consider creating a backup by copying the .db file.");

        connection
            .execute("DROP TABLE IF EXISTS Todo;", [])
            .unwrap();

        connection
            .execute(
                "CREATE TABLE Todo (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL);",
                [],
            )
            .unwrap();

        let init = Task {
            id: 0,
            title: "Initial task".to_string(),
        };

        connection.execute("INSERT INTO Todo (title) VALUES (?1)", [&init.title])?;

        println!("Successfully initialized the database.");
    }

    Ok(())
}
