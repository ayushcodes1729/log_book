use chrono::{self};
use std::fs::File;
use std::{fs, io};
use std::io::{Read, Write};

#[derive(PartialEq)]
enum Choice {
    ReadLogs,
    AddLogs,
    DeleteLogs,
    Exit,
}

fn read_logs() -> String {
    let logs = fs::read_to_string("./log_book.txt").expect("Expected logs inside log book");
    logs
}

fn write_logs(content: &str) {
    let mut file = fs::OpenOptions::new().write(true).append(true).read(true).open("./log_book.txt").expect("A file should be open in append");
    let current = chrono::offset::Local::now().to_string();
    let date_parts: Vec<&str> = current.split(" ").collect();
    let to_write = String::from("Date: ") + date_parts[0] + " " + date_parts[1] + "\n" + content + "\n";
    if let Err(e) = writeln!(file, "{to_write}") {
        eprintln!("Couldn't write to the file {}", e);
    };

    println!("Written")
}

fn delete_logs(log_date: &str) {
    let logs = fs::read_to_string("./log_book.txt").expect("Expected logs inside log book");
    let mut i = 0;
    match logs.find(log_date) {
        Some(index) => i = index,
        None => println!("Can't find log of the date")
    }

    let log_to_end = &logs[i..];

    let line_collection: Vec<&str> = log_to_end.split("\n").collect();

    let req_logs = String::from(line_collection[0]) + "\n" + line_collection[1];
    println!("{}",req_logs);

    
    let mut file = File::options().write(true).read(true).open("./log_book.txt").expect("A file should be opened in read and write mode");

    let mut buf = String::new(); 
    file.read_to_string(&mut buf).unwrap();
    drop(file);

    let mut file2 = File::options().write(true).truncate(true).open("./log_book.txt").expect("A file is expected");

    
    buf = buf.replace(&req_logs, "");
    
    file2.write_all(buf.as_bytes()).unwrap();

    drop(file2);
    println!("Deleted log")
}
fn main() {
    let mut user_choice: String = Default::default();

    let mut choice: Choice = Choice::ReadLogs;

    while choice != Choice::Exit {
        
        println!("Select one of the following options:\n1. Read\n2. Create\n3. Delete\n4. Exit");

        user_choice.clear();

        let _user_inp = io::stdin()
            .read_line(&mut user_choice)
            .expect("Expected a user input with choice");

        user_choice = user_choice.trim().to_lowercase();
        if user_choice == "read" {
            choice = Choice::ReadLogs;
        } else if user_choice == "create" {
            choice = Choice::AddLogs;
        } else if user_choice == "delete" {
            println!("Enter \"Date <Date from logs> \"");
            choice = Choice::DeleteLogs;
        } else if user_choice == "exit" {
            choice = Choice::Exit;
        } else {
            println!("Wrong choice!")
        }

        match choice {
            Choice::ReadLogs => {
                let read_log = read_logs();
                println!("{read_log}");
            }
            Choice::AddLogs => {
                let mut log_entry = Default::default();
                io::stdin().read_line(&mut log_entry).expect("Expect content to enter in the log_book");
                write_logs(&log_entry);
                println!("Log written");
            }
            Choice::DeleteLogs => {
                let mut log_date = Default::default();
                io::stdin().read_line(&mut log_date).expect("Expected a number corresponding a log");
                delete_logs(&log_date);
                println!("Log deleted");
            }
            Choice::Exit => {
                break;
            }
        }
    }

    println!("Diary logged")
}
