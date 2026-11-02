//! Local loopback streaming address manager and byte-range stream routing.

pub struct StreamEndpoint {
    pub port: u16,
    pub host: String,
}

impl Default for StreamEndpoint {
    fn default() -> Self {
        Self::new("127.0.0.1", 8766)
    }
}

impl StreamEndpoint {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    /// Construct loopback HTTP stream URL for a given episode index.
    pub fn stream_url(&self, episode: u32) -> String {
        format!("http://{}:{}/stream/ep{}", self.host, self.port, episode)
    }
}
