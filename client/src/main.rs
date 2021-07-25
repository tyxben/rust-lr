extern crate rand;
use rand::Rng;

use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let mut stream = TcpStream::connect("127.0.0.1:8099")?;
   

    for _ in 0..10 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        let mut trimmed_input = input.trim().clone().to_string();
        println!("{}",trimmed_input);
        let rng_str = rng.gen_range(0, 1000).to_string();
        
        trimmed_input.push_str("-challenge-");
        trimmed_input.push_str(&rng_str);

        stream
            .write(trimmed_input.as_bytes())
            .expect("Failed to write to stream");

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();

        //stream.read(&mut buffer);
        //let mut line = String::new();
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        println!(
            "{}",
            str::from_utf8(&buffer).expect("Could not write buffer as string")
        );
        //println!("{}", line);
        println!("sucess!");
    }
    Ok(())
}
