use std::env;
use std::fs::{self, File};
use std::path::Path;

fn main() {
    const FILENAME: &str = "schedule.txt";
    const MONTH_NAMES: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(command) => {
            match command.as_str() {
                "add" => {
                    let mut file;

                    if Path::new(FILENAME).exists() {
                        file = match File::open(FILENAME) {
                            Ok(x) => x,
                            Err(_) => {
                                println!("Failed opening file");
                                return;
                            }
                        }
                    } else {
                        file = match File::create(FILENAME) {
                            Ok(x) => x,
                            Err(_) => {
                                println!("Failed creating file");
                                return;
                            }
                        }
                    }

                    let string_date = match args.get(2) {
                        Some(x) => x,
                        None => {
                            println!("You need to provide a date");
                            return;
                        }
                    };

                    let splited: Vec<&str> = string_date.split('-').collect();    

                    let date_infos = match splited.first() {
                        Some(x) => x,
                        None => {
                            println!("Failed getting date infos");
                            return;
                        }
                    };
                    let date_details = match splited.get(1) {
                        Some(x) => x,
                        None => ""
                    };
                }
                "list" => {
                    match fs::read_to_string(FILENAME) {
                        Ok(content) => println!("{}", content),
                        Err(_) => {
                            println!("Failed reading file");
                            return;
                        }
                    }
                }
                _ => {
                    println!("Unknown command '{}'", command);
                    return;
                }
            }
        }
        None => {
            println!("Not enough arguments were given");
            return;
        }
    }
}
