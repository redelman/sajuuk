# Sajuuk

A Kubernetes-native plugin-based loadbalancer written in Rust.

## Features

- Plugin-based architecture
  - VIP type / protocol plugins (HTTP, HTTP_QUIC, TCP, UDP)
  - Loadbalancing algorithm plugins (round robin, least connections, weighted round robin)
  - Plugin dependency management
  - Plugin versioning and metadata
- Dynamic configuration via API
- Comprehensive metrics via OpenTelemetry

## Development

### Prerequisites

- Rust 1.82 or later
- Docker (for building and testing)

### Quick Start

1. Clone the repository:

   ```bash
   git clone https://github.com/redelman/sajuuk.git
   cd sajuuk
   ```

2. Open in Dev Container:
   - VSCode will automatically detect the Dev Container configuration
   - Click "Reopen in Container" when prompted
   - The container will set up all necessary development tools

3. Build the project:

   ```bash
   cargo build
   ```

4. Run tests:

   ```bash
   cargo test
   ```

## Running

```bash
# Run with default config
sajuuk

# Run with specific config file
sajuuk --config /path/to/config.yaml

# Run in background
sajuuk --daemon

# Stop running instance
sajuuk --stop
```

## Configuration

Sajuuk uses YAML for configuration. The default configuration file location is `/etc/sajuuk/config.yaml`.

Example configuration:

```yaml
metrics:
  address: "127.0.0.1"
  port: 9090

backend_sets:
  web_servers:
    algorithm: round_robin
    backends:
      - address: "10.0.0.1"
        port: 8080
      - address: "10.0.0.2"
        port: 8080

vips:
  - address: "192.168.1.10"
    port: 80
    type: tcp
    backend_set: web_servers
```

## License

Apache License 2.0

## Author

Rich Edelman <redelman@gmail.com>
