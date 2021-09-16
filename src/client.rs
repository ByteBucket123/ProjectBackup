use anyhow::{Context, Result};
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(addr: &str) -> Self {
        Client {
            stream: TcpStream::connect(addr)
                .context(format!(
                    "Client failed to establish connection to addr: {}",
                    addr
                ))
                .unwrap(),
        }
    }

    pub fn send_message(&mut self, message: &[u8]) -> Result<()> {
        self.stream.write(message)?;
        Ok(())
    }

    pub fn get_response(&mut self) -> Result<[u8; 1024]> {
        let mut buffer = [0; 1024];
        self.stream.read(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_file_to_server(&mut self, file_path: &str) -> Result<()> {
        let mut file_vec = Vec::new();
        file_vec.copy_from_slice(b"file");
        let mut file_contents: Vec<u8> = fs::read(file_path)?;
        file_vec.append(&mut file_contents);
        self.stream.write(&file_contents[..])?;
        Ok(())
    }

    pub fn read_file_from_server_and_write_to_disk(
        &mut self,
        destination_path: &str,
    ) -> Result<()> {
        let response = self.get_response()?;
        fs::write(destination_path, response)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
