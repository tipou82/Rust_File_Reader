use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{self, Write};

fn main() {
    
    print!("Enter filename: ");
    io::stdout().flush().unwrap();
    
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Failed to read line");
    let filename = filename.trim();
    
    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}