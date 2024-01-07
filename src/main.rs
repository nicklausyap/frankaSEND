// Date created: Feb 19, 2023
// This source file is part of the frankaSEND open source project
// Copyright (c) 2023 nicklaus yap ken yik

use frankasend::ThreadPool;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::io::Read;
use local_ip_address::local_ip;

fn main() {
    //let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!(" d'b                    8             .oPYo.                   8     ");
    println!(" 8                      8             8                        8     ");
    println!("o8P  oPYo. .oPYo. odYo. 8  .o  .oPYo. `Yooo. .oPYo. odYo. .oPYo8     ");
    println!(" 8   8  `' .oooo8 8' `8 8oP'   .oooo8     `8 8oooo8 8' `8 8    8     ");
    println!(" 8   8     8    8 8   8 8 `b.  8    8      8 8.     8   8 8    8     ");
    println!(" 8   8     `YooP8 8   8 8  `o. `YooP8 `YooP' `Yooo' 8   8 `YooP'     ");
    println!(":..::..:::::.....:..::....::...:.....::.....::.....:..::..:.....:    ");
    println!(":::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::    ");
    println!(":::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::    ");
    let my_local_ip = local_ip().unwrap();
    println!("Currently running at local IP address: {:?}:7878", my_local_ip);
    // using current machine ip address
    let listener = TcpListener::bind(format!("{:?}:7878",my_local_ip.to_owned())).unwrap();
    let pool = ThreadPool::new(4);
    
    // add while loop to keep it running forever
    while true{
        for stream in listener.incoming().take(2) {
            let stream = stream.unwrap();

            pool.execute(|| {
            handle_connection(stream);
            });
        }
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let gettextfile = b"GET /hello.txt HTTP/1.1\r\n";
    let getzipfile = b"GET /frankailoveu.zip HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(getzipfile) {
        ("HTTP/1.1 200 OK", "frankailoveu.zip")
    } else if buffer.starts_with(gettextfile) {
        ("HTTP/1.1 200 OK", "hello.txt")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    // handle binary file. Example binary file is zip
    let mut f = File::open(filename).expect("unable to open");
    let mut contents = Vec::new();
    f.read_to_end(&mut contents);

    // normal way to handle files
    // let contents = fs::read_to_string(filename).unwrap();
    
    // if no == 1{
        let response = format!(
            "{}\r\nContent-Disposition: attachment; filename=\"{}\"\r\nContent-Type: application/zip\r\nContent-Transfer-Encoding: binary\r\nContent-Length: {}\r\n\r\n",
            status_line,
            filename,
            contents.len()
        );

    // }else{
    //     // let response = format!(
    //     //     "{}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
    //     //     status_line,
    //     //     contents.len(),
    //     //     contents
    //     // );
    // }

    // stream.write_all(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
    stream.write_all(response.as_bytes());
    stream.write_all(&contents);
    stream.flush();
}
