# Telegram Agent Experiment

## Project Overview

A privacy-focused Telegram data collection agent that operates as a human-like AI assistant (user account, not bot) to gather anonymized data from public Telegram groups and channels for analysis purposes.

## Key Features

- **Human-like Operation**: Runs as a user account, not a bot account
- **Read-Only Access**: Only reads messages from public groups/channels it has joined
- **Privacy-First**: GDPR compliant with anonymized data collection
- **CLI-Based**: Simple command-line tool designed for easy daemonization
- **Data Pipeline Integration**: Sends collected data to data lake pipeline
- **Lightweight**: Minimal dependencies, designed for long-running operations

## Technical Stack

- **Language**: Rust (using idiomatic conventions)
- **Telegram Library**: grammers (v0.8.1 from git)
- **Async Runtime**: tokio
- **Deployment**: Systemd-compatible daemon

## Architecture

### Design Principles

1. **Simplicity**: Keep the codebase simple and maintainable
2. **Reliability**: Designed to run continuously as a daemon
3. **Privacy**: All data collection is anonymized and GDPR compliant
4. **Read-Only**: No message sending, only passive observation

### Core Components

```
tg-agent-experiment/
├── src/
│   ├── main.rs           # CLI entry point and daemon setup
│   ├── client.rs         # Telegram client initialization
│   ├── collector.rs      # Message collection logic
│   ├── anonymizer.rs     # Data anonymization
│   └── pipeline.rs       # Data lake pipeline integration
├── Cargo.toml
└── CLAUDE.md
```

## Privacy & Compliance

### GDPR Compliance

- **Data Minimization**: Only collect necessary message data
- **Anonymization**: Remove personally identifiable information (PII)
- **Public Data Only**: Only access publicly available groups/channels
- **No User Tracking**: Do not track individual user behavior
- **Transparent Operation**: Clear documentation of data collection practices

### Data Anonymization Strategy

The agent will anonymize:
- User IDs (hash or remove)
- Usernames (remove or pseudonymize)
- Phone numbers (remove)
- Profile photos (not collected)
- Location data (not collected)

Retained for analysis:
- Message content (text only, sanitized)
- Timestamps (rounded to hour/day)
- Channel/group metadata (public information only)
- Message metadata (reply chains, forwarding patterns)

## Usage

### Running as CLI

```bash
# Run in foreground
cargo run -- --config config.toml

# Run with logging
RUST_LOG=info cargo run -- --config config.toml
```

### Running as Systemd Service

```bash
# Build release binary
cargo build --release

# Copy binary
sudo cp target/release/tg-agent-experiment /usr/local/bin/

# Setup systemd service (see deployment docs)
sudo systemctl enable tg-agent
sudo systemctl start tg-agent
```

## Development Guidelines

### Rust Conventions

- Follow official Rust style guide (rustfmt)
- Use `cargo clippy` for linting
- Write comprehensive error handling with `Result<T, E>`
- Prefer idiomatic Rust patterns:
  - Use iterators over loops where appropriate
  - Leverage ownership system for memory safety
  - Use `async/await` for concurrent operations
  - Minimize `unsafe` code (avoid if possible)

### Code Quality

- Write unit tests for core logic
- Document public APIs with doc comments
- Keep functions small and focused
- Use meaningful variable names
- Add comments for complex logic only

## Configuration

The agent will be configured via TOML file:

```toml
[telegram]
api_id = 123456
api_hash = "your_api_hash"
session_file = "session.db"

[collector]
channels = ["channel1", "channel2"]
groups = ["group1", "group2"]

[pipeline]
endpoint = "https://data-lake.example.com/api/ingest"
batch_size = 100
flush_interval = 60  # seconds

[privacy]
anonymize_users = true
retain_timestamps = true
timestamp_precision = "hour"  # hour, day, week
```

## Roadmap

### Phase 1: Core Implementation (Current)
- [ ] Telegram client setup and authentication
- [ ] Basic message reading from channels
- [ ] Message data structure definition
- [ ] Simple logging output

### Phase 2: Data Pipeline
- [ ] Data anonymization implementation
- [ ] Pipeline integration
- [ ] Batch processing
- [ ] Error handling and retries

### Phase 3: Production Readiness
- [ ] Configuration file support
- [ ] Systemd service integration
- [ ] Comprehensive logging
- [ ] Monitoring and health checks
- [ ] Documentation and deployment guide

### Future Enhancements
- Message filtering and categorization
- Multiple data pipeline support
- Advanced anonymization techniques
- Rate limiting and backpressure handling

## Security Considerations

1. **API Credentials**: Store securely, never commit to git
2. **Session Files**: Protect session files (contain auth tokens)
3. **Network Security**: Use TLS for all external communications
4. **Access Control**: Run with minimal system privileges
5. **Logging**: Avoid logging sensitive information

## License

[To be determined]

## Contributing

This is a personal project. Contributions guidelines will be added if opened to community.

## Contact

[To be determined]
