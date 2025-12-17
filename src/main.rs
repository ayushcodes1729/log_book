use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut result_string= String::from("");
    for s in args {
        if s == "target/debug/todo_list" {
            continue;
        }
        result_string+=&s;
        result_string+=&String::from(" ");
    }
    println!("{}", result_string)
}
