use std::io::Read;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.first() {
        Some(command) => {
            match command {
                _ => {
                    "add" => {

                    }
                    "list" => {

                    }
                    _ => {
                        println!("Unknown command '{}'", command);
                        return;
                    }
                }
            }
        }
        None => {
            println!("Not enough arguments were given");
            return;
        }
    }

    const FILENAME: &str = "schedule.txt";
    const MONTH_NAMES: [&str: 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let mut file;

    if Path::new(FILENAME).exists() {
        file = File::open(FILENAME).expect("Failed opening file");
    } else {
        file = File::create(FILENAME).expect("Failed creating file");
    }

    let mut content = String::new();
    file.read_to_string(&mut content);

    println!("File content: {}", content);
}
