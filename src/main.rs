#![allow(dead_code)]
// use std::convert::TryFrom;
use std::env;
// use std::process;
// use std::thread;
// use std::time::Duration;

// use console::Term;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Entry {
    id: i32,
    task: String,
}

// impl Entry {
//     fn new(args: &[String]) -> Result<Entry, &str>) {
//         for i in args {

//         }
//     }
// }

fn get_valid_entries(input: &Vec<String>) -> String {
    let mut entry_list: String = String::from("");
    let mut counter: i32 = 0;
    println!("{}", input.len());
    for x in input.iter() {
        if counter > 2 {
            let s = x.as_str();
            entry_list.push_str(s);
            entry_list.push_str(" ");
        }
        counter += 1;
    }
    entry_list.pop();

    entry_list
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // let term = Term::stdout();
    let conn = Connection::open("todo_list.db")?;
    let valid_args: String = get_valid_entries(&args);
    println!("TESTING: {}", valid_args);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (id INTEGER PRIMARY KEY AUTOINCREMENT, entry TEXT NOT NULL)",
        [],
    )?;

    if args.len() < 2 {
        println!("too small");
    } else if args[1] == "todo" {
        if args.len() == 2 {
        } else if args[2] == "add" {
            println!("add stuff");
            // add( rest of args );
            conn.execute("INSERT INTO entries (entry) VALUES (?1)", [valid_args])?;
        } else if args[2] == "done" {
            println!("done stuff");
            // done( rest of args );
        } else if args[2] == "remove" {
            // FIXME: add ability to delete multiple entries at one time
            conn.execute("DELETE FROM entries WHERE id = (?1)", [valid_args])?;
        }
    }

    // if let Err(e) = term.write_line("Hello World!") {
    //     eprintln!("Application error: {}", e);
    //     process::exit(1);
    // }

    // thread::sleep(Duration::from_secs(1));

    // if let Err(e) = term.clear_line() {
    //     eprintln!("Application error: {}", e);
    //     process::exit(1);
    // }

    let mut stmt = conn.prepare("SELECT id, entry FROM entries")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Entry {
            id: row.get(0)?,
            task: row.get(1)?,
        })
    })?;

    for person in person_iter {
        println!("{:?}", person.unwrap());
    }

    Ok(())
}
