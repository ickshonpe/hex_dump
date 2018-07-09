use std::error::Error;

fn main() {
    let mut data: Vec<u8> =
        if let Some(file_name) = std::env::args().nth(1) {
            std::fs::read(&file_name).unwrap_or_else(|e| {
                eprintln!("Unable to open file {}", file_name);
                eprintln!("Error: {}", e.description());
                std::process::exit(1);
            })
        } else {
            eprintln!("usage: hex-dump <file-name>");
            std::process::exit(2);
        };
    data.reverse();
    let mut index = 0;
    while !data.is_empty() {
        let mut hex_output = String::new();
        let mut chars_output = String::new();
        for _ in 0 .. 10 {
            if let Some(next_byte) = data.pop() {
                hex_output += &format!("{:02x} ", next_byte);
                let char_code = u32::from(if 31 < next_byte && next_byte < 127 { next_byte } else { 32 });
                chars_output.push( unsafe { std::char::from_u32_unchecked(char_code ) });

            }
            else {
                break;
            }
        }
        println!("{:08x}: {:<34} {}", index, hex_output, chars_output);
        index += 10;
    }
}


