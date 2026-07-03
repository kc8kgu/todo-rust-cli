use std::env;
use std::process;


struct Obj {
    name: String,
    value: String
}

// static PATH: &str = "./todo.csv";

fn main() {

    let argv: Vec<String> = env::args().collect();

    if argv.len() == 1 {
        usage();
        process::exit(0);
    } else if argv.len() == 2 && argv[1] == "list" {
        list_todo();
    } else if argv.len() == 3 && argv[1] == "add" {
        add_todo(&argv[2]);
    } else if argv.len() == 4 && argv[1] == "edit"{
        edit_todo(&argv[2], &argv[3]);
    } else if argv.len() == 3 && argv[1] == "done" {
        done_todo(&argv[2]);
    } else if argv.len() == 3 && argv[1] == "delete" {
        delete_todo(&argv[2]);
    } else if argv.len() == 2 && argv[1] == "clear" {
        clear_todo();
    } else {
        println!("Incorrect parameters or unknown command: {}.", argv[1]);
        usage();
    }

    process::exit(0);
}

fn usage() {
    println!("Usage: td <list|add|edit|done|delete> <id> <title>");
}

fn list_todo() {
    println!("listing todos");

    let todos: Vec<Vec<Obj>> = load_todos();

    for todo in todos {
        for obj in todo {
            println!("{}:{}", obj.name, obj.value);
        }
    }
}

fn add_todo(_title: &String) {
    println!("adding todos");
}

fn edit_todo(_id: &String, _title: &String) {
    println!("editing todos");}

fn done_todo(_id: &String) {
    println!("done todos");}

fn delete_todo(_id: &String) {
    println!("deleting todos");}

fn clear_todo() {
    println!("clearing todos");
}

fn load_todos() -> Vec<Vec<Obj>> {
    println!("loading todos");

    let todos: Vec<Vec<Obj>> = Vec::new();
    //let mut todo: Vec<Obj> = Vec::new();

    todos
}

/*
fn load_todos_test() -> Vec<Vec<Obj>>{
    println!("loading todos");

    let mut todos: Vec<Vec<Obj>> = Vec::new();

    let mut todo: Vec<Obj> = Vec::new();
    todo.push( Obj { name: "id".to_string(), value: "0".to_string() } );
    todo.push( Obj { name: "done".to_string(), value: "0".to_string() } );
    todo.push( Obj { name: "title".to_string(), value: "check in to git".to_string() } );
    todos.push(todo);

    let mut todo: Vec<Obj> = Vec::new();
    todo.push( Obj { name: "id".to_string(), value: "0".to_string() } );
    todo.push( Obj { name: "done".to_string(), value: "0".to_string() } );
    todo.push( Obj { name: "title".to_string(), value: "check in to git".to_string() } );
    todos.push(todo);


    todos
}
*/

/*
fn save_todos() {
    println!("saving todos");
}
*/
