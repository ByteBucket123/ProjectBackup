use std::io::{Read, Write};

struct MockStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

impl MockStream {
    fn new() ->  MockStream  {
        MockStream {
            read_data: vec![],
            write_data: vec![],
            }
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Vec::append(&mut self.read_data, &mut buf.to_vec());
        Ok(0)
    }
}

impl Write for MockStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Vec::append(&mut self.write_data, &mut buf.to_vec());
        Ok(0)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}