use readbin::headers::elf;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match fs::read(&args[1]) {
            Ok(data) => match elf::x64::from_bytes(&data) {
                Some(header) => println!("{}", header),
                None => println!("Failed to parse elf"),
            },
            Err(err) => println!("Error reading binary: {}", err),
        },
        _ => {
            println!("usage: {:?} <binary file>", args[0]);
        }
    }
}
