use chrono::{Local, NaiveDate};
use clap::{Arg, ArgMatches, Command};
use console::style;
use dirs;
use dotenv::dotenv;
use notification::send_notification;
use rusqlite::{params, Connection, Result};
use std::process;
mod notification;

const DB_FILE: &str = ".todo_list.db";

fn get_db_path() -> String {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(DB_FILE);
    path.to_string_lossy().to_string()
}

fn init_db() -> Result<Connection> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0,
            date TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(conn)
}

fn insert_todo(task: &str, end_date: &str) {
    let conn = init_db().expect("Failed to initialize database");

    conn.execute(
        "INSERT INTO todos (task, done, date) VALUES (?1, 0, ?2)",
        params![task, end_date],
    )
    .expect("Failed to insert todo");

    send_notification("add");
}

fn is_task_expired(date: &str) -> bool {
    let today = Local::now().date_naive(); // Get only the date part

    // Parse the input date string as NaiveDate
    let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .expect("Invalid date format. Expected YYYY-MM-DD");

    today > naive_date
}

fn display_on_console() {
    let conn = init_db().expect("Failed to initialize database");

    let mut stmt = conn
        .prepare("SELECT id, task, done, date FROM todos")
        .expect("Failed to fetch tasks");
    let tasks = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            let task: String = row.get(1)?;
            let done: bool = row.get(2)?;
            let date: Option<String> = row.get(3)?;
            Ok((id, task, done, date))
        })
        .expect("Failed to parse tasks");

    for task in tasks {
        let (id, task, done, date) = task.expect("Error reading task");
        if done {
            println!(
                "{}",
                style(format!("{}: {}", id, task)).red().strikethrough()
            );
        } else if is_task_expired(&date.expect("Problem with date")) {
            println!("{}", style(format!("⚠️ Task is overdue!")).yellow());
        } else {
            println!("{}", style(format!("{}: {}", id, task)).green());
        }
    }
}

fn mark_as_done(task_id: &str) {
    let conn = init_db().expect("Failed to initialize database");

    conn.execute("UPDATE todos SET done = 1 WHERE id = ?1", params![task_id])
        .expect("Failed to update todo");

    send_notification("done");
}

fn delete_content() {
    let db_path = get_db_path();
    let conn = Connection::open(db_path).expect("Problem with connection in delete_content.");

    conn.execute("DROP TABLE IF EXISTS todos", [])
        .expect("Problem with dropping the table.");

    println!("{}", style("All tasks deleted!").red());
    send_notification("delete");
}

fn sync_in_cloud() {
    let db_path = get_db_path();
    let cloud_path = std::env::var("CLOUD_PATH").expect("CLOUD_PATH must be set.");
    let output_command = process::Command::new("rclone")
        .arg("copy")
        .arg(db_path)
        .arg(cloud_path)
        .arg("--update")
        .output()
        .expect("Failet to execute rclone command");

    if output_command.status.success() {
        println!("✅ Sync successful!");
    } else {
        eprintln!(
            "❌ Sync failed: {}",
            String::from_utf8_lossy(&output_command.stderr)
        );
    }
}

fn sync_in_local() {
    let path = dirs::home_dir().expect("Could not find home directory");
    let local_path = std::env::var("LOCAL_PATH").expect("LOCAL_PATH must be set.");

    let output_command = process::Command::new("rclone")
        .arg("copy")
        .arg(local_path)
        .arg(path)
        .arg("--update")
        .output()
        .expect("Failet to execute rclone command");

    if output_command.status.success() {
        println!("✅ Sync successful!");
    } else {
        eprintln!(
            "❌ Sync failed: {}",
            String::from_utf8_lossy(&output_command.stderr)
        );
    }
}

fn execute_command(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let task = sub_matches
                .get_one::<String>("task")
                .expect("Task is required");
            let due_date = sub_matches
                .get_one::<String>("due")
                .expect(&"No due date".to_string());
            insert_todo(&task, &due_date);
        }
        Some(("delete", _)) => {
            delete_content();
        }
        Some(("done", sub_matches)) => {
            let task_id = sub_matches
                .get_one::<String>("task")
                .expect("Task that should be marked as done is required.");

            mark_as_done(&task_id);
        }
        Some(("sync-local", _)) => {
            sync_in_local();
        }
        Some(("sync-cloud", _)) => {
            sync_in_cloud();
        }
        Some(("list", _)) => {
            display_on_console();
        }
        _ => println!("No matches"),
    }
}

fn init() {
    let matches = Command::new("todo")
        .subcommand(
            Command::new("add")
                .about("Add a new todo task.")
                .arg(
                    Arg::new("task")
                        .required(true)
                        .help("The task description."),
                )
                .arg(
                    Arg::new("due")
                        .long("due")
                        .short('d')
                        .help("The due date for the task."),
                ),
        )
        .subcommand(
            Command::new("done").about("Mark task as done.").arg(
                Arg::new("task")
                    .required(true)
                    .help("The task description."),
            ),
        )
        .subcommand(Command::new("delete").about("Delete all tasks.").alias("r"))
        .subcommand(Command::new("sync-local").about("Sync Cloud data in local."))
        .subcommand(Command::new("sync-cloud").about("Sync local data in cloud."))
        .subcommand(
            Command::new("list")
                .about("List the current tasks.")
                .alias("-l"),
        )
        .get_matches();

    execute_command(&matches);
}

fn main() {
    dotenv().ok();
    init();
}
