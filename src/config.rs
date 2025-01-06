use std::{env, net::Ipv4Addr};

#[derive(Debug)]
pub struct Config {
    bind_address: Ipv4Addr,
    bind_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        println!("Initializing config from environment.");

        let raw_address = env::var("BIND_ADDRESS").unwrap_or("127.0.0.1".to_string());
        let bind_address = raw_address.parse::<Ipv4Addr>();
        if bind_address.is_err() {
            return Err(format!(
                "Failed to convert `{raw_address}` to an IP address: `{}`",
                bind_address.unwrap_err()
            ));
        }
        let bind_address = bind_address.unwrap();

        let raw_port = env::var("BIND_PORT").unwrap_or("1337".to_string());
        let bind_port = raw_port.parse::<u16>();
        if bind_port.is_err() {
            return Err(format!(
                "Failed to convert `{raw_port}` to a valid port number: `{}`",
                bind_port.unwrap_err()
            ));
        }
        let bind_port = bind_port.unwrap();

        Ok(Self {
            bind_address,
            bind_port,
        })
    }

    pub fn bind_address(&self) -> &Ipv4Addr {
        &self.bind_address
    }

    pub fn bind_port(&self) -> &u16 {
        &self.bind_port
    }
}
