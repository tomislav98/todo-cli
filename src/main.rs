use console::style;
use dirs;
use dotenv::dotenv;
use std::env;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn get_path_file() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".todo_list.txt");
    if !path.exists() {
        File::create(&path).expect("Failed to create todo_list.txt");
    }
    path
}

fn get_num_lines() -> usize {
    let file_content = match read_to_string(get_path_file()) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return 0;
        }
    };
    file_content.lines().count()
}

fn insert_todo(items: Vec<String>) {
    let path = get_path_file();

    let mut data_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .expect("Cannot open file");

    let num_lines = get_num_lines();

    for (index, item) in items.iter().enumerate() {
        let item_with_new_line = format!("{} {}\n", num_lines + index + 1, item);
        data_file
            .write_all(item_with_new_line.as_bytes())
            .expect("Write failed");
    }
}

fn delete_content() {
    let path = get_path_file();
    File::create(path).expect("Content of file cannot be deleted");
}

fn display_on_console() {
    let path = get_path_file();

    if let Ok(content) = read_to_string(path) {
        for line in content.lines() {
            if line.contains("DONE") {
                println!("{}", style(line).red().strikethrough());
            } else {
                println!("{}", style(line).green());
            }
        }
    } else {
        eprintln!("Todo list file does not exist.");
    }
}

fn create_file_when_marked(new_content: &str) {
    delete_content();
    let path = get_path_file();

    let mut data_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .expect("Cannot open file");

    data_file
        .write_all(new_content.as_bytes())
        .expect("Write failed");
}

fn mark_as_done(items: Vec<String>) {
    let path = get_path_file();

    let file_content = match read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return;
        }
    };

    let mut new_file = file_content.clone();
    for item in items.iter() {
        let new_content = format!("DONE: {}", item);
        if let Some(pos) = new_file.find(item) {
            new_file.replace_range(pos..pos + item.len(), &new_content);
        }
    }
    create_file_when_marked(&new_file);
}

fn init() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        display_on_console();
    } else {
        match args[1].to_lowercase().as_str() {
            "add" => insert_todo(args[2..].to_vec()),
            "delete" => delete_content(),
            "done" => mark_as_done(args[2..].to_vec()),
            _ => eprintln!("Invalid command. Use 'add', 'delete', or 'done'."),
        }
    }
}

fn main() {
    dotenv().ok();
    init();
}
