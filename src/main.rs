use std::{env, fs};
use chrono;
// use fs;``

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut result_string= String::from("");
    let current = chrono::offset::Local::now().to_string(); 
    println!("{current}");
    let date_parts: Vec<&str> = current.split(" ").collect();
    
    for s in args {
        if s == "target/debug/log_book" {
            continue;
        }
        result_string+=&s;
        result_string+=&String::from(" ");
    }
    println!("String from inputs {}", result_string);
    
    let to_write = String::from(date_parts[0]) + " " + date_parts[1] + "\n" + &result_string;

    let content = fs::write("./log_book.txt", &to_write).expect("Content of the files should be written");
    
    println!("Diary logged")
}
