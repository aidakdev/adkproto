use std::{
    path::PathBuf,
    net::SocketAddr,
};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub server_url: String,
    pub bind_address: SocketAddr,
    pub data_dir: Option<PathBuf>,
    pub federation_enabled: bool,
    pub known_peers: Vec<String>
}

impl ServerConfig {
    pub fn new(server_url: String, bind_address: SocketAddr) -> Self {
        Self {
            server_url,
            bind_address,
            data_dir: None,
            federation_enabled: true,
            known_peers: Vec::new(),
        }
    }
    
    pub fn with_data_dir(mut self, data_dir: PathBuf) -> Self {
        self.data_dir = Some(data_dir);
        self
    }
    
    pub fn with_federation(mut self, enabled: bool) -> Self {
        self.federation_enabled = enabled;
        self
    }
    
    pub fn with_peers(mut self, peers: Vec<String>) -> Self {
        self.known_peers = peers;
        self
    }
} 