use anyhow::{Context, Result};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use std::thread;
use std::time::Duration;

static HELLO_HTML: &'static str = include_str!("res/hello.html");
static ERR_HTML: &'static str = include_str!("res/404.html");

pub trait ServerFunctionality {
    fn new(listener: TcpListener, pool: ThreadPool) -> Self;

    fn run(&self) -> Result<()>;

    fn stop(&self) -> Result<()>;
}
pub struct Server {
    pool: ThreadPool,
    listener: TcpListener,
}

impl ServerFunctionality for Server {
    fn new(listener: TcpListener, pool: ThreadPool) -> Self {
        Server {
            listener: listener,
            pool: pool,
        }
    }

    fn run(&self) -> Result<()> {
        for stream in self.listener.incoming() {
            let stream = stream?;

            self.pool.spawn(|| {
                handle_connection(stream).unwrap_or_else(|err| {
                    eprintln!("Error handling stream: {}", err);
                });
            });
        }
        Ok(())
    }

    fn stop(&self) -> Result<()> {
        Ok(())
    }
}

fn handle_connection(mut stream: impl Read + Write) -> Result<()> {
    println!("Task executes on thread: {:?}", thread::current().id());

    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .context("Failed to read bytes from stream")?;

    let s = str::from_utf8(&buffer).context("Failed to read string from buffer")?;
    println!("Received request:\n{}", s);

    let (status_line, contents) = parse_stream_buffer(&mut buffer);

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn parse_stream_buffer(buffer: &[u8]) -> (&str, &'static str) {
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", HELLO_HTML)
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", HELLO_HTML)
    } else {
        ("HTTP/1.1 404 NOT FOUND", ERR_HTML)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind, Read, Write};

    use anyhow::{bail, Result};
    use mockall::mock;

    mock! {
        TcpStream {}
        impl Read for TcpStream{
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
        }
        impl Write for TcpStream{
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
            fn flush(&mut self) -> std::io::Result<()>;
        }
    }

    mock! {
        TcpListener {}
    }

    mock! {
        ThreadPool{}
    }

    fn new_default_mock_tcp_stream(read: bool, write: bool, flush: bool) -> MockTcpStream {
        let mut mock_stream = MockTcpStream::new();
        mock_stream.expect_read().return_once(move |_| {
            if read {
                Ok(0)
            } else {
                Err(Error::new(ErrorKind::Other, "This MockStream can't read!"))
            }
        });
        mock_stream.expect_write().return_once(move |_| {
            if write {
                Ok(0)
            } else {
                Err(Error::new(ErrorKind::Other, "This MockStream can't write!"))
            }
        });
        mock_stream.expect_flush().return_once(move || {
            if flush {
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Other, "This MockStream can't flush!"))
            }
        });
        mock_stream
    }

    #[test]
    //   fn simpel_server() -> Result<()> {
    //       let addr = "0.0.0.0::7878";
    //       let nbr_threads = 4;
    //       let mock_tcp_listener = MockTcpListener::new();
    //       let mock_thread_pool = MockThreadPool::new();
    //       let server = Server::new(mock_tcp_listener, mock_thread_pool);
    //       server.run()?;
    //       server.stop()?;
    //       Ok(())
    //   }

    //  #[test]
    //  fn not_simpel_server() -> Result<()> {
    //  let addr = "0.0.0.0::7878";
    //    let nbr_threads = 4;
    //let mock_tcp_listener = MockTcpListener::new();
    //let mock_thread_pool = MockThreadPool::new();
    //      let server = Server::new(mock_tcp_listener, mock_thread_pool);
    //        server.run()?;
    //    let mock
    //      //Get server response
    //  server.stop()?;
    //Ok(())
    //}
    #[test]
    fn handle_good_connection() -> Result<()> {
        let mock_stream = new_default_mock_tcp_stream(true, true, true);
        handle_connection(mock_stream)
    }

    #[test]
    fn handle_bad_read_connection() -> Result<()> {
        let mock_stream = new_default_mock_tcp_stream(false, true, true);
        let result = handle_connection(mock_stream);
        match result {
            Ok(_) => bail!("We bailed in the test"),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn handle_bad_write_connection() -> Result<()> {
        let mock_stream = new_default_mock_tcp_stream(true, false, true);
        let result = handle_connection(mock_stream);
        match result {
            Ok(_) => bail!("We bailed in the test"),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn handle_bad_flush_connection() -> Result<()> {
        let mock_stream = new_default_mock_tcp_stream(true, true, false);
        let result = handle_connection(mock_stream);
        match result {
            Ok(_) => bail!("We bailed in the test"),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn parse_404_buffer() {
        let bad = b"bad";
        assert_eq!(
            ("HTTP/1.1 404 NOT FOUND", ERR_HTML),
            parse_stream_buffer(bad)
        );
    }

    #[test]
    fn parse_get_buffer() {
        let get = b"GET / HTTP/1.1\r\n";
        assert_eq!(("HTTP/1.1 200 OK", HELLO_HTML), parse_stream_buffer(get));
    }

    #[test]
    #[ignore]
    fn parse_sleep_buffer() {
        let sleep = b"GET /sleep HTTP/1.1\r\n";
        assert_eq!(("HTTP/1.1 200 OK", HELLO_HTML), parse_stream_buffer(sleep));
    }
}
