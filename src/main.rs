use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
}

fn tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!");
    }
}

fn tcp_client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} 

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tcp_server_test() {
        tcp_server();
    }

    #[test]
    fn tcp_client_test() {
        let _res = tcp_client();
    }
}
