use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [2; 99];
    let mut str1 = String::new();
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        println!(
            "{}",
            str::from_utf8(&buf).expect("Could not write buffer as string")
        );

        str1.push_str(str::from_utf8(&buf).expect("Could not"));
        str1.push_str( "-server-sucess\n");
        stream.write(str1.as_bytes())?;
        //stream.write(&mut buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1 as u64));
        println!("new message1");
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8099")?;

    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("failed");
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
        });

        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
