use std::net::TcpListener;
use std::io::{BufReader, Read, Write};

struct Config {
    protocol: String,
    port: u16,
    host: String,
}

fn error(msg: &str) {
    eprintln!("Error - {}", msg);
}

impl Config {


    fn tcp(&self) {
        println!("Starting TCP server on {}:{}", self.host, self.port);

        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap_or_else(|err| {
            error(&format!("Failed to bind to {}:{} - {}", self.host, self.port, err));
            std::process::exit(1);
        });

        for stream in listener.incoming() {
            let mut stream = stream.unwrap_or_else(|err| {
                error(&format!("Failed to accept connection - {}", err));
                std::process::exit(1);
            });

            std::thread::spawn(move || {
                println!("Connection established");

                let mut reader = BufReader::new(&mut stream);

                let mut final_data = String::new();
                loop {
                    let mut buffer = [0; 1024];
                    let bytes_read = reader.read(&mut buffer).unwrap_or_else(|err| {
                        error(&format!("Failed to read from stream - {}", err));
                        std::process::exit(1);
                    });



                    let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                    final_data.push_str(&data);

                    if bytes_read < 1024 {
                        break;
                    }
                }

                println!("Received: {}", final_data);

                stream.write(final_data.as_bytes()).unwrap_or_else(|err| {
                    error(&format!("Failed to write to stream - {}", err));
                    std::process::exit(1);
                });
            });
            
        }
    }
}

fn error_usage() {
    eprintln!("Usage: echos <protocol> <port> <host>");
    std::process::exit(1);
}

fn main() {
    let raw_arguments = std::env::args().collect::<Vec<String>>();

    let config = Config {
        protocol: match raw_arguments.get(1) {
            Some(protocol) => {
                match protocol.as_str() {
                    "tcp" => protocol.to_string(),
                    _ => {
                        eprintln!("Error - Unsupported protocol: {}", protocol);
                        std::process::exit(1);
                    }
                }
            },
            None => {
                error_usage();
                std::process::exit(1);
            }
        },

        port: match raw_arguments.get(2) {
            Some(port) => {
                match port.parse::<u16>() {
                    Ok(port) => port,
                    Err(err) => {
                        eprintln!("Error - Invalid port: {}", err);
                        std::process::exit(1);
                    }
                }
            },
            None => {
                error_usage();
                std::process::exit(1);
            }
        },

        host: match raw_arguments.get(3) {
            Some(host) => {
                match host.split(".").collect::<Vec<&str>>().len() {
                    4 => host.to_string(),
                    _ => {
                        eprintln!("Error - Invalid host: {}", host);
                        std::process::exit(1);
                    }
                }
            },
            None => {
                error_usage();
                std::process::exit(1);
            }
        }
    };

    match config.protocol.as_str() {
        "tcp" => config.tcp(),
        _ => {
            error(&format!("Unsupported protocol: {}", config.protocol));
            std::process::exit(1);
        }
    }
}

