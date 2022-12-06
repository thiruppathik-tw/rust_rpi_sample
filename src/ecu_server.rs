use std::clone;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{io, thread, time::Duration};

use crate::occupancy;

#[allow(unused)]
enum EcuError {
    UnknownRequest,
    FunctionalError(occupancy::OccupancyError),
}

fn get_request_word(request: &String, i: usize) -> (String, usize) {
    let req: Vec<&str> = request.split("/").collect();
    println!("{:?}", req);
    let r = req.get(i).unwrap_or(&"Invalid");
    let mut r = String::from(*r);
    r.retain(|c| !c.is_whitespace());
    (r, req.len())
}

fn request_handler(mut stream: TcpStream, request: &String) -> Result<(), EcuError> {
    let r = get_request_word(request, 0);
    println!("{}, {}", r.0, r.1);
    match r.0.as_str() {
        "GET" => {
            let mut s = occupancy::device_info();
            if r.1 > 0 {
                let p0 = get_request_word(request, 1);
                match p0.0.as_str() {
                    "DEV" => {
                        println!("Get DeviceInfo: {}", s);
                    }
                    "SENSOR" => {
                        s = {
                            match occupancy::occupancy_status() {
                                Ok(s) => s.to_string(),
                                Err(_) => "Unknown".to_string(),
                            }
                        };
                    }
                    _ => {
                        println!("Get DeviceInfo: {}", s);
                    }
                }
            }
            println!("[request_handler] Writing to stream.");
            let _ = stream.write(s.as_bytes());
        }
        _ => {
            println!("Unknown");
            return Err(EcuError::UnknownRequest);
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), EcuError> {
    let mut buffer = String::new();
    for i in 1..10 {
        match stream.read_to_string(&mut buffer) {
            Ok(_) => {
                println!("string: {}", buffer);
                return request_handler(stream, &buffer);
            }
            Err(_) => {
                println!("handle_client: Error({})", i);
                continue;
            }
        }
    }
    Ok(())
}

pub fn server_init() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let s = listener.accept();

    match s {
        Ok((stream, _addr)) => {
            println!("[server_init]");
            let _r = handle_client(stream);
            Ok(())
        }
        Err(_) => todo!(),
    }

    // accept connections and process them serially
    // for stream in listener.incoming() {
    //     let _ = handle_client(stream?);
    // }
    // Ok(())
}
