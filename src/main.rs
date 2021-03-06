mod event;

use std::io::Write;
use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::cmp::Ordering;

use chrono::{Datelike, Local};

use event::Event;

fn main() {
    const MONTH_NAMES: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let file_name = &format!("{}\\events.json", home::home_dir().unwrap().to_str().unwrap());
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(command) => {
            match command.as_str() {
                "add" => {
                    let args: Vec<&str> = args.iter()
                        .skip(2)
                        .map(|x| x.as_str())
                        .collect();

                    if args.len() < 2 {
                        println!("Not enough arguments were given");
                        return;
                    }

                    let mut content = String::new();
                   
                    if Path::new(file_name).exists() {
                        content = match fs::read_to_string(file_name) {
                            Ok(x) => x,
                            Err(_) => {
                                println!("Failed reading file");
                                return;
                            }
                        };
                    }

                    let mut events: Vec<Event> = match serde_json::from_str(&content) {
                        Ok(x) => x,
                        Err(_) => vec![]
                    };

                    let event = match Event::from_string(args[0], args[1]) {
                        Ok(x) => x,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };

                    let mut event_found = false;

                    for e in events.iter_mut() {
                        if event == *e {
                            e.message = event.message.clone();
                            event_found = true;
                            
                            break;
                        }
                    }

                    if !event_found {
                        events.push(event);
                    }

                    let string_json = serde_json::to_string(&events).unwrap();

                    let mut file = match File::create(file_name) {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Failed creating file");
                            return;
                        }
                    };
                        
                    file.write_all(string_json.as_bytes()).expect("Failed writing to file");
                }
                "remove" => {
                    let args: Vec<&str> = args.iter()
                        .skip(2)
                        .map(|x| x.as_str())
                        .collect();

                    if args.is_empty() {
                        println!("A date is needed");
                        return;
                    } else if args.len() > 1 {
                        println!("Too many arguments were given");
                        return;
                    }

                    let event = match Event::from_string(args[0], "") {
                        Ok(x) => x,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };

                    let content = match fs::read_to_string(file_name) {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Failed reading file");
                            return;
                        }
                    };

                    let mut events: Vec<Event> = match serde_json::from_str(&content) {
                        Ok(x) => x,
                        Err(_) => vec![]
                    };

                    for (i, e) in events.iter().enumerate() {
                        if event == *e {
                            events.remove(i);
                            break;
                        }
                    }

                    let string_json = serde_json::to_string(&events).unwrap();

                    let mut file = match File::create(file_name) {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Failed creating file");
                            return;
                        }
                    };
                        
                    file.write_all(string_json.as_bytes()).expect("Failed writing to file");
                }
                "clear" => {
                    fs::remove_file(file_name).unwrap();
                }
                "list" => {
                    let content = match fs::read_to_string(file_name) {
                        Ok(x) => x,
                        Err(_) => {
                            println!("Failed reading file");
                            return;
                        }
                    };

                    let mut events: Vec<Event> = match serde_json::from_str(&content) {
                        Ok(x) => x,
                        Err(_) => vec![]
                    };

                    events.sort();

                    let mut year = 0;
                    let mut month = 0;

                    for event in events {
                        if year != event.year || month != event.month {
                            year = event.year;
                            month = event.month;

                            println!("\n{} = {} =", event.year, MONTH_NAMES[(event.month - 1) as usize]);
                        }

                        let info_char;
                        let date_now = Local::now();

                        match event.year.cmp(&(date_now.year() as u16)) {
                            Ordering::Equal => {
                                match event.month.cmp(&(date_now.month() as u8)) {
                                    Ordering::Equal => {
                                        match event.day.cmp(&(date_now.day() as u8)) {
                                            Ordering::Equal => info_char = '*',
                                            Ordering::Less => info_char = '-',
                                            Ordering::Greater => info_char = '+'
                                        }
                                    }
                                    Ordering::Less => info_char = '-',
                                    Ordering::Greater => info_char = '+'
                                }
                            }
                            Ordering::Less => info_char = '-',
                            Ordering::Greater => info_char = '+'
                        }

                        println!("{} [{}] ({}:{}.{}) - {}", info_char, event.day, event.hour, event.minute, event.second, event.message);
                    }
                }
                _ => println!("Unknown command '{}'", command)
            }
        }
        None => println!("Not enough arguments were given")
    }
}
