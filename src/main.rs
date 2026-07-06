use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

const PATH: &str = "todo.json";

#[derive(Serialize, Deserialize)]
struct Todo {
    id: u64,
    done: bool,
    title: String,
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() == 2 && argv[1] == "list" {
        list_todos();
    } else if argv.len() == 3 && argv[1] == "add" {
        add_todo(&argv[2]);
    } else if argv.len() == 4 && argv[1] == "edit" {
        edit_todo(&argv[2], &argv[3]);
    } else if argv.len() == 3 && argv[1] == "done" {
        done_todo(&argv[2]);
    } else if argv.len() == 3 && argv[1] == "delete" {
        delete_todo(&argv[2]);
    } else if argv.len() == 2 && argv[1] == "clear" {
        clear_todos();
    } else {
        usage();
    }
}

fn usage() {
    println!("Usage: td < list | add | edit | done | delete | clear > <id> <title>");
}

fn list_todos() {
    let todos: Vec<Todo> = load_todos();

    for todo in todos {
        println!("{}, {}, {}", todo.id, todo.done, todo.title);
    }
}

fn add_todo(title: &str) {
    let mut todos = load_todos();

    let mut max_id = 0;
    for todo in &todos {
        if todo.id > max_id {
            max_id = todo.id;
        }
    }

    let next_id = max_id + 1;
    let new_todo = Todo {
        id: next_id,
        done: false,
        title: title.to_string(),
    };

    todos.push(new_todo);
    save_todos(&todos);
}

fn edit_todo(id: &str, title: &str) {
    let mut todos = load_todos();
    let id = id.parse::<u64>().unwrap();
    let mut found = false;

    for todo in &mut todos {
        if todo.id == id {
            todo.title = title.to_string();
            found = true;
            break;
        }
    }

    if found {
        save_todos(&todos);
    } else {
        println!("id not found");
    }
}

fn done_todo(id: &str) {
    let mut todos = load_todos();
    let id = id.parse::<u64>().unwrap();
    let mut found = false;

    for todo in &mut todos {
        if todo.id == id {
            todo.done = !todo.done;
            found = true;
            break;
        }
    }

    if found {
        save_todos(&todos);
    } else {
        println!("id not found");
    }
}

fn delete_todo(id: &str) {
    let mut todos = load_todos();
    let id = id.parse::<u64>().unwrap();
    let mut found = false;

    for (index, todo) in &mut todos.iter().enumerate() {
        if todo.id == id {
            todos.remove(index);
            found = true;
            break;
        }
    }

    if found {
        save_todos(&todos);
    } else {
        println!("id not found");
    }
}

fn clear_todos() {
    let todos = Vec::new();
    save_todos(&todos);
}

fn load_todos() -> Vec<Todo> {

    let todos: Vec<Todo>;

    if Path::exists(Path::new(PATH)) {
        let reader: BufReader<File> = BufReader::new(File::open(PATH).unwrap());
        todos = serde_json::from_reader(reader).unwrap();
    } else {
        todos = Vec::new();
    }

    todos
}

fn save_todos(todos: &Vec<Todo>) {
    let writer: BufWriter<File> = BufWriter::new(File::create(PATH).unwrap());
    //serde_json::to_writer(writer, todos).unwrap();
    serde_json::to_writer(writer, todos).unwrap();
}

// fn save_todos(todos: &Vec<Todo>) -> Result<(), std::io::Error>{
//     let writer: BufWriter<File> = BufWriter::new(File::create(PATH)?);
//     serde_json::to_writer(writer, todos)?;
//
//     Ok(())
// }
