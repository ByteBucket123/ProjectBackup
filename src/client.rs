use anyhow::{Context, Result};
use std::net::TcpStream;

pub fn start() {
    panic!("Client not yet implemented!");
}

pub fn establish_connection(addr: &str) -> Result<TcpStream> {
    TcpStream::connect(addr).context(format!(
        "Client failed to establish connection to addr: {}",
        addr
    ))
}

#[cfg(test)]
mod tests {
    use {
        anyhow::{bail, Error},
        std::{
            io::Write,
            net::{TcpListener, TcpStream},
            thread,
        },
    };

    use super::*;

    struct MockServer<T> {
        server_thread: thread::JoinHandle<T>,
    }

    fn create_mock_server() -> thread::JoinHandle<()> {
        thread::spawn(run_mock_server)
    }

    fn run_mock_server() {
        let mut stream = TcpListener::bind("0.0.0.0:7878")
            .unwrap()
            .incoming()
            .next()
            .unwrap()
            .unwrap();
        stream.write(b"MockServer").unwrap();
        stream.flush().unwrap();
    }

    #[test]
    #[ignore]
    fn test_establish_connection() -> Result<(), Error> {
        let addr = "127.0.0.1:7878";
        match establish_connection(addr) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[test]
    fn test_fail_connection() -> Result<(), Error> {
        let addr = "Bad addr";
        match establish_connection(addr) {
            Ok(_) => bail!("Connection should be error!"),
            Err(_) => Ok(()),
        }
    }
}
