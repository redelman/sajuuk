mod types;

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub use types::*;

/// Load configuartion from a YAML file
pub fn load_config<P>(path: P) -> Result<Config>
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

    serde_yaml::from_str(&contents)
        .with_context(|| format! {"Failed to parse config file: {}", path.as_ref().display()})
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_config() -> Result<()> {
        let dir = tempdir()?;
        let config_path = dir.path().join("config.yaml");
        let mut file = File::create(&config_path)?;

        write!(
            file,
            r#"
metrics:
  address: "127.0.0.1"
  port: 9090
backend_sets:
  web_servers:
    algorithm: round_robin
    backends:
      - address: "10.0.0.1"
        port: 8080
vips:
  - address: "192.168.1.10"
    port: 80
    type: tcp
    backend_set: web_servers
"#
        )?;

        let config = load_config(config_path)?;
        assert_eq!(config.metrics.port, 9090);
        assert!(config.backend_sets.contains_key("web_servers"));
        assert_eq!(config.vips.len(), 1);

        Ok(())
    }
}
