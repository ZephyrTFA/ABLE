use std::{env, net::Ipv4Addr};

use log::trace;

#[derive(Debug)]
pub struct Config {
    bind_address: Ipv4Addr,
    bind_port: u16,
    rate_limit_burst: u32,
    rate_limit_per_second: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        trace!("Initializing config from environment.");

        let raw_address = env::var("BIND_ADDRESS").unwrap_or("127.0.0.1".to_string());
        let bind_address = raw_address.parse();
        if bind_address.is_err() {
            return Err(format!(
                "Failed to convert `{raw_address}` to an IP address: `{}`",
                bind_address.unwrap_err()
            ));
        }
        let bind_address = bind_address.unwrap();

        let raw_port = env::var("BIND_PORT").unwrap_or("1337".to_string());
        let bind_port = raw_port.parse();
        if bind_port.is_err() {
            return Err(format!(
                "Failed to convert `{raw_port}` to a valid port number: `{}`",
                bind_port.unwrap_err()
            ));
        }
        let bind_port = bind_port.unwrap();

        let raw_rlb = env::var("BIND_PORT").unwrap_or("5".to_string());
        let rate_limit_burst = raw_rlb.parse();
        if rate_limit_burst.is_err() {
            return Err(format!(
                "Failed to convert `{raw_rlb}` to a valid number: `{}`",
                rate_limit_burst.unwrap_err()
            ));
        }
        let rate_limit_burst = rate_limit_burst.unwrap();

        let raw_rls = env::var("BIND_PORT").unwrap_or("1337".to_string());
        let rate_limit_per_second = raw_rls.parse();
        if rate_limit_per_second.is_err() {
            return Err(format!(
                "Failed to convert `{raw_rls}` to a valid number: `{}`",
                rate_limit_per_second.unwrap_err()
            ));
        }
        let rate_limit_per_second = rate_limit_per_second.unwrap();

        Ok(Self {
            bind_address,
            bind_port,
            rate_limit_burst,
            rate_limit_per_second,
        })
    }

    pub fn bind_address(&self) -> &Ipv4Addr {
        &self.bind_address
    }

    pub fn bind_port(&self) -> u16 {
        self.bind_port
    }

    pub fn rate_limit_burst(&self) -> u32 {
        self.rate_limit_burst
    }

    pub fn rate_limit_per_second(&self) -> u64 {
        self.rate_limit_per_second
    }
}
