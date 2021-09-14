use anyhow::{Context, Result, bail};
use std::net::{TcpListener, TcpStream};

pub fn start() {
    panic!("Client not yet implemented!");
}

pub fn establish_connection(addr: &str) -> Result<TcpStream> {
    TcpStream::connect(addr).context(format!("Client failed to establish connection to addr: {}", addr))
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use super::*;

    #[test]
    fn test_establish_connection() -> Result<(), Error> {
        let addr = "127.0.0.1:7878";
        match establish_connection(addr) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
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
