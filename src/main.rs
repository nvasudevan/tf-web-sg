use chrono::Utc;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut tcp_str: TcpStream, app_env: &str, app_ver: &str) {
    let client = tcp_str.peer_addr().unwrap().to_string();
    println!("connection received from {}", client);
    let mut buf = [0; 1024];
    if let Ok(_) = tcp_str.read(&mut buf) {
        println!("client msg: {}", String::from_utf8_lossy(&buf[..]));
        let now = Utc::now().format("%Y-%m-%d-%H-%M-%S%.3f");
        let contents = format!("{} {} {}", now, app_env, app_ver);
        let res_hdr = format!("HTTP/1.1 200 OK\r\nContent-Length: {}", contents.len());
        let res = format!("{}\r\n\r\n{}\n", res_hdr, contents);
        match tcp_str.write(res.as_bytes()) {
            Ok(_) => {
                println!("=> response sent to client: {}", client);
            }
            Err(e) => {
                println!(
                    "=> error writing to client: {}, error: {}",
                    client,
                    e.to_string()
                );
            }
        }
        tcp_str.flush().unwrap();
    };
}

fn main() {
    let app_env = std::env::var("APP_ENV").expect("APP_ENV environment variable is not set!");
    let app_ver = std::env::var("APP_VER").expect("APP_VER environment variable is not set!");
    let bind_addr = "0.0.0.0:8080";
    println!("=> starting web server on {}", bind_addr);
    let my_web = TcpListener::bind(bind_addr).unwrap();

    for stream in my_web.incoming() {
        match stream {
            Ok(tcp_str) => handle_connection(tcp_str, &app_env, &app_ver),
            Err(e) => {
                println!("Connection failed: {}", e.to_string());
            }
        }
    }
}
