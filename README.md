# chexx0r

> Fast CLI tool to check domain and social media username availability

## Quick Start

```bash
# Install
cargo install --path .

# Check a username
chexx0r yungalgo

# With custom TLDs
chexx0r myapp --tlds com,io,dev
```

## What It Checks

**Domains** (via RDAP + WHOIS):
- Uses presets: `startup`, `enterprise`, `country`
- Or custom TLDs: `--tlds com,net,org`

**Social Media**:
- YouTube
- Instagram  
- TikTok

## Installation

See [BUILD.md](BUILD.md) for complete build and installation instructions.

## Example

```
🔍 Checking availability for: yungalgo
============================================================

📡 DOMAIN AVAILABILITY
------------------------------------------------------------
yungalgo.com                   AVAILABLE
yungalgo.org                   AVAILABLE
yungalgo.io                    AVAILABLE
yungalgo.ai                    AVAILABLE

📱 SOCIAL MEDIA AVAILABILITY
------------------------------------------------------------
YouTube                        TAKEN
Instagram                      AVAILABLE
TikTok                         AVAILABLE

============================================================
✅ Check complete!
```

## Usage

```bash
# Basic check with startup preset
chexx0r myusername

# Enterprise TLDs
chexx0r mybrand --preset enterprise

# Custom TLDs
chexx0r myapp --tlds com,net,dev,io

# Only domains
chexx0r myapp --skip-social

# Only social media
chexx0r myapp --skip-domains

# Help
chexx0r --help
```

## Built With

- [domain-check-lib](https://github.com/saidutt46/domain-check) - Domain availability checking
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client for social media checks
- [clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime

## License

MIT

