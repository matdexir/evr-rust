use chrono::{serde::ts_seconds, DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read, Write},
};

#[derive(Debug, Deserialize, Serialize)]
enum Urgency {
    Low,
    Mid,
    High,
    Extreme,
}

#[derive(Debug, Serialize)]
struct Todo {
    task: String,
    #[serde(with = "ts_seconds")]
    creation: DateTime<Utc>,
    urgency: Urgency,
    #[serde(with = "ts_seconds")]
    deadline: DateTime<Utc>,
    completed: bool,
}

#[derive(Debug)]
struct Todos {
    items: Vec<Todo>,
}

fn main() -> Result<()> {
    // TODO:
    // Find a way to incorporate filepath
    // let filepath = String::from("./");
    let filename = String::from("data.json");
    let mut f = File::create(&filename).unwrap();
    let mut todos = Todos { items: Vec::new() };
    let end_message = String::from("end");
    println!(
        "Hi my man can you please input a todo:(type {} to finish)",
        end_message
    );
    loop {
        let mut todo = String::new();
        io::stdin()
            .read_line(&mut todo)
            .expect("Hey broski you better give me something");
        let todo: String = match todo.trim().parse() {
            Ok(todo) => todo,
            Err(_) => continue,
        };
        if todo == end_message {
            println!("Stopping input");
            break;
        }
        loop {
            let mut deadline = String::new();
            let datetime_fmt = String::from("%Y-%m-%d %H:%M");
            println!("Please enter a deadline:(format {})", datetime_fmt);
            io::stdin()
                .read_line(&mut deadline)
                .expect("Give me some solid date");
            let deadline: String = match deadline.trim().parse() {
                Ok(deadline) => deadline,
                Err(_) => continue,
            };
            let deadline = NaiveDateTime::parse_from_str(&deadline, &datetime_fmt).unwrap();
            println!("{:?}", deadline);
            let new_todo = Todo {
                task: todo.clone(),
                deadline: Utc.from_local_datetime(&deadline).unwrap(),
                completed: false,
                creation: Utc::now(),
                urgency: Urgency::Low,
            };
            todos.items.push(new_todo);
            break;
        }
    }
    println!("Thanks");
    println!("Your todos are:");
    for (i, todo) in todos.items.iter().enumerate() {
        let j = serde_json::to_string(&todo)?;
        println!("{}- {:?}", i, j.to_string());
        f.write_all(j.to_string().as_bytes()).unwrap();
    }
    println!("That's it for today");
    Ok(())
}
