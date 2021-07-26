#[derive(Clone)]
pub struct ServiceOptions {
    pub address: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub port: Option<u16>,
    pub graceful_shutdown: bool,
    pub keep_alive: u32,
    pub read_timeout: u32,
    pub write_timeout: u32,
}

impl Default for ServiceOptions {
    fn default() -> Self {
        ServiceOptions {
            address: Some("127.0.0.1".to_string()),
            name: Some("north-service".to_string()),
            version: Some("latest".to_string()),
            port: Some(5000),
            graceful_shutdown: false,
            keep_alive: 1,
            read_timeout: 2,
            write_timeout: 2,
        }
    }
}
