use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use serde::{Deserialize, Serialize};

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
        println!("ok");
    } else if argv.len() == 4 && argv[1] == "edit"{
        edit_todo(&argv[2], &argv[3]);
        println!("ok");
    } else if argv.len() == 3 && argv[1] == "done" {
        done_todo(&argv[2]);
        println!("ok");
    } else if argv.len() == 3 && argv[1] == "delete" {
        delete_todo(&argv[2]);
        println!("ok");
    } else if argv.len() == 2 && argv[1] == "clear" {
        clear_todos();
        println!("ok");
    } else {
        usage();
    }
}

fn usage() {
    println!("Usage: td < list | add | edit | done | delete> <id> <title>");
}

fn list_todos() {

    let todos: Vec<Todo> = load_todos();

    for todo in todos {
        println!("{}, {}, {}", todo.id, todo.done, todo.title);
    }

}

fn add_todo(title: &String) {
    let mut todos = load_todos();
    
    let mut max_id = 0;
    for todo in &todos {
        if todo.id > max_id {
            max_id = todo.id;
        }
    }

    let next_id = max_id + 1;
    let new_todo = Todo {id: next_id, done: false, title: title.clone()};
    
    todos.push(new_todo);
    save_todos(&todos);
}

fn edit_todo(_id: &String, _title: &String) {

    let mut todos = load_todos();
    let id = _id.parse::<u64>().unwrap();

    for todo in &mut todos {
        if todo.id == id {
            todo.title = _title.clone();
            break;
        }
    }

    save_todos(&todos);
}

fn done_todo(_id: &String) {

    let mut todos = load_todos();
    let id = _id.parse::<u64>().unwrap();

    for todo in &mut todos {
        if todo.id == id {
            todo.done = !todo.done;
            break;
        }
    }

    save_todos(&todos);
}

fn delete_todo(_id: &String) {

    let mut todos = load_todos();
    let id = _id.parse::<u64>().unwrap();

    for (index, todo) in &mut todos.iter().enumerate() {
        if todo.id == id {
            todos.remove(index);
            break;
        }
    }    
    
    save_todos(&todos);
}

fn clear_todos() {

    let todos = Vec::new();
    save_todos(&todos);
}

fn load_todos() -> Vec<Todo> {

    let reader: BufReader<File> = BufReader::new(File::open(PATH).unwrap());
    let todos: Vec<Todo> = serde_json::from_reader(reader).unwrap();

    todos
}


fn save_todos(todos: &Vec<Todo>) {

    let writer: BufWriter<File> = io::BufWriter::new(File::create("todo.json").unwrap());
    serde_json::to_writer(writer, todos).unwrap();
}