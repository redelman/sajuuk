use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;

/// Main config structure
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// Metrics server configuration
    #[serde(default)]
    pub metrics: MetricsConfig,
    /// Backend sets configuration
    pub backend_sets: HashMap<String, BackendSet>,
    /// VIP configurations
    pub vips: Vec<Vip>,
}

/// Metrics server configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// IP address to bind metrics server to
    #[serde(default = "default_address")]
    pub address: IpAddr,
    /// Port to bind metrics server to
    #[serde(default = "default_metrics_port")]
    pub port: u16,
}

/// Backend Sets configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct BackendSet {
    /// Load balancing algorithm to use
    pub algorithm: Algorithm,
    /// List of backends in this set
    pub backends: Vec<Backend>,
}

/// Backend configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Backend {
    /// Backend IP address
    pub address: IpAddr,
    /// Backend port
    pub port: u16,
    // Optional backend weight for weighted algorithms
    #[serde(default = "default_weight")]
    pub weight: u32,
    /// Optional maximum number of concurrent connections
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

/// VIPs configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Vip {
    /// VIP IP address
    pub address: IpAddr,
    /// VIP Port
    pub port: u16,
    /// Protocol type
    #[serde(rename = "type")]
    pub protocol_type: ProtocolType,
    // Associated backend set name
    pub backend_set: String,
}

/// Load Balancing Algorithms
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Algorithm {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IpHash,
}

/// Supported protocols
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProtocolType {
    Tcp,
    Udp,
}

fn default_address() -> IpAddr {
    "127.0.0.1".parse().unwrap()
}

fn default_metrics_port() -> u16 {
    9090
}

fn default_weight() -> u32 {
    100
}

fn default_max_connections() -> u32 {
    1000000
}

impl Default for MetricsConfig {
    fn default() -> Self {
        MetricsConfig {
            address: default_address(),
            port: default_metrics_port(),
        }
    }
}

// impl Default for Config {
//     fn default() -> Self {
//         Config {
//             metrics: MetricsConfig::default(),
//             backend_sets: HashMap::new(),
//             vips: Vec::new(),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_config_serialization() {
        let config = Config {
            metrics: MetricsConfig {
                address: IpAddr::from_str("127.0.0.1").unwrap(),
                port: 9090,
            },
            backend_sets: {
                let mut map = HashMap::new();
                map.insert(
                    "web_servers".to_string(),
                    BackendSet {
                        algorithm: Algorithm::RoundRobin,
                        backends: vec![Backend {
                            address: IpAddr::from_str("10.0.0.1").unwrap(),
                            port: 8080,
                            weight: 100,
                            max_connections: 10000,
                        }],
                    },
                );
                map
            },
            vips: vec![Vip {
                address: IpAddr::from_str("192.168.1.10").unwrap(),
                port: 80,
                protocol_type: ProtocolType::Tcp,
                backend_set: "web_servers".to_string(),
            }],
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: Config = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(deserialized.metrics.port, config.metrics.port);
        assert_eq!(deserialized.vips.len(), config.vips.len());
    }

    #[test]
    fn test_metrics_defaults() {
        let yaml = "backend_sets: {}\nvips: []";
        let config: Config = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(config.metrics.address.to_string(), "127.0.0.1");
        assert_eq!(config.metrics.port, 9090);
    }
}
