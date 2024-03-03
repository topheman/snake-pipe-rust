use std::net::TcpListener;

#[derive(Debug, Clone)]
pub struct UrlToDisplay {
    pub url: String,
}

impl UrlToDisplay {
    pub fn new(port: u16) -> UrlToDisplay {
        if let Ok(ip) = local_ip_address::local_ip() {
            Self {
                url: format!("http://{}:{}", ip, port),
            }
        } else {
            return Self {
                url: format!("http://localhost:{}", port),
            };
        }
    }
}

pub fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("0.0.0.0", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
