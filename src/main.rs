use std::error::Error;

fn main() {
    let mut data: Vec<u8> =
        if let Some(file_name) = std::env::args().skip(1).next() {
            std::fs::read(&file_name).unwrap_or_else(|e| {
                eprintln!("Unable to open file {}", file_name);
                eprintln!("Error: {}", e.description());
                std::process::exit(1);
            })
        } else {
            eprintln!("usage: hex-dump <file-name> ");
            std::process::exit(2);
        };
    data.reverse();
    let mut index = 0;
    while !data.is_empty() {
        print!("{:09}: ", index);
        for _ in 0 .. 10 {
            if let Some(next_byte) = data.pop() {
                print!("{:02x}  ", next_byte);
            }
            else {
                break;
            }
        }
        println!();
        index += 10;
    }
}


