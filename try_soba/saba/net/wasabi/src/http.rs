extern crate alloc;
use alloc::vec::Vec;
use alloc::format;
use alloc::string::{String, ToString};
use noli::net::{SocketAddr, TcpStream, lookup_host};
use saba_core::error::Error;
use saba_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl HttpClient {
    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        let ips = match lookup_host(&host) {
            Ok(ips) => ips, // １つのホストに対して複数のIPアドレスが見つかる可能性がある（ロードバランサーが複数あるケースなど）ので配列。
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to find IP Addresses: {:#?}",
                    e
                )))
            }
        };
        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP Addresses".to_string()));
        }

        let socket_addr: SocketAddr = (ips[0], port).into();

        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_) => {
                return Err(Error::Network("Faild to connect TCP stream".to_string()))
            }
        }

        // build a request line
        let mut request = String::from("GET /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\n");

        // add header
        request.push_str("Host: ");
        request.push_str(&host);
        request.push_str('\n');
        request.push_str("Accept: text/html\n");
        request.push_str("Connection: close\n");
        request.push_str('\n');

        let _bytes_written = match stream.write(request.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(Error::Network("Failed to send a request to TCP stream.".to_string()))
            }
        }

        // handle response data
        let mut received = Vec::new();
        loop {
            let mut buf = [0u8, 4096];
            let bytes_read = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::Network("Failed to receive a request from TCP stream.".to_string()))
                }
            };
            if bytes_read == 0 {
                break;
            }
            received.extend_from_slice(&buf[..bytes_read]);
        }

        // decode bytes reponse data to string
        match core::str::from_utf8(&received) {
            Ok(response) => HttpResponse::new(response.to_string()),
            Err(e) => Err(Error::Network(format!("Invalid received response: {}", e)))
        }
    }
}
