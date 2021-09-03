use chrono::Utc;
use std::{
    env,
    io::{Read, Write, Result},
    net::{TcpListener, TcpStream},
};
use tokio::net::UdpSocket;
use std::net::SocketAddr;

/// Send an udp message `app_env=X;app_version=Y` to the `udp_server`
async fn send_udp_msg(udp_server: &str, app_env: &str, app_ver: &str) -> Result<()> {
    let msg = format!("app_env={};app_version={}", app_env, app_ver);
    let local_addr: SocketAddr = SocketAddr::V4("0.0.0.0:0".parse().unwrap());
    let skt = UdpSocket::bind(local_addr).await?;
    if let Err(e) = skt.send_to(msg.as_bytes(), udp_server).await{
       eprintln!("=> error writing to udp server: {}", e);
    }

    Ok(())
}

fn handle_connection(mut tcp_str: TcpStream, app_env: &str, app_ver: &str) -> Result<String> {
    let client = tcp_str.peer_addr()?;
    println!("connection received from {}", client.to_string());
    let mut buf = [0; 1024];
    if let Ok(_) = tcp_str.read(&mut buf) {
        let now = Utc::now().format("%Y-%m-%d-%H-%M-%S%.3f");
        let contents = format!("{} {} {}", now, app_env, app_ver);
        let res_hdr = format!("HTTP/1.1 200 OK\r\nContent-Length: {}", contents.len());
        let res = format!("{}\r\n\r\n{}\n", res_hdr, contents);
        if let Err(e) = tcp_str.write(res.as_bytes()) {
            eprintln!("=> error writing to client {} ({})", client, e.to_string());
        }

        tcp_str.flush()?;
    };

    Ok(client.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let app_env = env::var("APP_ENV")
        .expect("APP_ENV environment variable is not set!");
    let app_ver = env::var("APP_VER")
        .expect("APP_VER environment variable is not set!");
    let bind_addr = env::var("BIND_ADDR")
        .expect("BIND_ADDR environment variable is not set!");
    let udp_server = env::var("UDP_SERVER")
        .expect("UDP_SERVER environment variable is not set!");
    println!("=> starting web server on {}", bind_addr);

    let tcp_srv = TcpListener::bind(bind_addr)?;
    for stream in tcp_srv.incoming() {
        match stream {
            Ok(tcp_str) => {
                match handle_connection(tcp_str, &app_env, &app_ver) {
                    Ok(_) => {
                        let _ = send_udp_msg(&udp_server, &app_env, &app_ver).await;
                    }
                    Err(e) => {
                        eprintln!("Failed to read from client: {}", e.to_string());
                    }
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e.to_string());
            }
        }
    }

    Ok(())
}
