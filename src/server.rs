use anyhow::{Context, Result};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::thread;
use std::time::Duration;

static HELLO_HTML: &'static str = include_str!("res/hello.html");
static ERR_HTML: &'static str = include_str!("res/404.html");

pub fn start() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.spawn(|| {
            handle_connection(stream).unwrap();
        });
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
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

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
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
