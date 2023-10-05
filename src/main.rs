#![allow(dead_code)]
// use std::convert::TryFrom;
use std::env;
// use std::process;
// use std::thread;
// use std::time::Duration;

// use console::Term;
use console::style;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Entry {
    id: i32,
    task: String,
    done: i32,
}

// impl Entry {
//     fn new(args: &[String]) -> Result<Entry, &str>) {
//         for i in args {

//         }
//     }
// }

fn get_valid_entries(input: &Vec<String>) -> Vec<String> {
    let mut entry_list: Vec<String> = Vec::new();
    let mut counter: i32 = 0;
    // println!("{}", input.len());
    for x in input.iter() {
        // println!("{}", x);
        if counter > 2 {
            entry_list.push(x.to_string());
        }
        counter += 1;
    }

    entry_list
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // let term = Term::stdout();
    let conn = Connection::open("todo_list.db")?;
    let valid_args: Vec<String> = get_valid_entries(&args);
    let valid_len = valid_args.len();
    // println!("TESTING: {}", valid_args[0]);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (id INTEGER PRIMARY KEY AUTOINCREMENT, entry TEXT NOT NULL, isDone INTEGER DEFAULT 0)",
        [],
    )?;

    if args.len() < 2 {
        println!("too small");
    } else if args[1] == "todo" {
        if args.len() == 2 {
            let mut stmt = conn.prepare("SELECT id, entry, isDone FROM entries")?;
            let person_iter = stmt.query_map([], |row| {
                Ok(Entry {
                    id: row.get(0)?,
                    task: row.get(1)?,
                    done: row.get(2)?,
                })
            })?;

            

            for person in person_iter {

                let doneness = person.as_ref().unwrap().done;
                print!("{:?}", person.as_ref().unwrap().id);
                // print!("{:?}", person.as_ref().unwrap().task);
                let ptask = &person.as_ref().unwrap().task;
                if doneness == 0 {
                    println!(" {}", style(ptask).cyan());
                } else {
                    println!(" {}", style(ptask).red());
                }

            }
        } else if args[2] == "add" {
            println!("add stuff");
            // add( rest of args );
            let mut counter = 0;
            while counter < valid_len {
                conn.execute("INSERT INTO entries (entry) VALUES (?1)", [&valid_args[counter]])?;
                counter+=1;
            }
        } else if args[2] == "done" {
            println!("done stuff");
            let mut counter = 0;
            while counter < valid_len {
                conn.execute(
                    "UPDATE entries SET isDone=1 WHERE id=(?1)",
                    [&valid_args[counter]],
                )?;

                counter+=1;
            }
        } else if args[2] == "rm" {
            let mut counter = 0;
            while counter < valid_len {
                conn.execute("DELETE FROM entries WHERE id = (?1)", [&valid_args[counter]])?;

                counter+=1;
            }
        }
    }

    // if let Err(e) = term.write_line("Hello World!") {
    //     eprintln!("Application error: {}", e);
    //     process::exit(1);
    // }

    // thread::sleep(Duration::from_secs(1));

    // if let Err(e) = term.clear_line() {
        // eprintln!("Application error: {}", e);
        // process::exit(1);
    // }

    Ok(())
}
