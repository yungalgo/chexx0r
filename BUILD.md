# chexx0r - Build & Installation Guide

## Overview

`chexx0r` is a Rust CLI tool that checks domain and social media username availability. It combines:
- **Domain checking** using [domain-check-lib](https://github.com/saidutt46/domain-check) (RDAP + WHOIS)
- **Social media checking** for YouTube, Instagram, and TikTok via HTTP requests

## Prerequisites

### 1. Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

Verify installation:

```bash
rustc --version
cargo --version
```

### 2. System Requirements

- **macOS**: 10.15+ (Catalina or later)
- **Memory**: 4GB+ RAM recommended
- **Disk Space**: ~2GB for Rust toolchain + dependencies

## Building from Source

### Step 1: Clone or Navigate to Project

```bash
cd /Users/yungalgo/Desktop/repos/chexx0r
```

### Step 2: Build the Project

**Development build** (faster compile, slower runtime):
```bash
cargo build
```

**Release build** (optimized, recommended):
```bash
cargo build --release
```

The binary will be located at:
- Development: `./target/debug/chexx0r`
- Release: `./target/release/chexx0r`

### Step 3: Test the Build

```bash
# Development
./target/debug/chexx0r test

# Release
./target/release/chexx0r test
```

## Installation

### Option 1: Install Locally (Recommended)

Install to `~/.cargo/bin/` (automatically in PATH):

```bash
cargo install --path .
```

Now you can run from anywhere:

```bash
chexx0r yungalgo
```

### Option 2: Symlink to /usr/local/bin

```bash
cargo build --release
sudo ln -sf "$(pwd)/target/release/chexx0r" /usr/local/bin/chexx0r
```

### Option 3: Add to PATH

```bash
# Build release version
cargo build --release

# Add to your shell config (~/.zshrc or ~/.bashrc)
echo 'export PATH="$PATH:/Users/yungalgo/Desktop/repos/chexx0r/target/release"' >> ~/.zshrc
source ~/.zshrc
```

## Usage Examples

### Basic Usage

Check a username with default startup preset (com, org, io, ai, tech, app, dev, xyz):

```bash
chexx0r yungalgo
```

### Custom TLD Preset

```bash
# Startup-focused TLDs (default)
chexx0r myapp --preset startup

# Enterprise-focused TLDs
chexx0r mybrand --preset enterprise

# Country-specific TLDs
chexx0r mycompany --preset country
```

### Custom TLDs

```bash
# Check specific TLDs only
chexx0r yungalgo --tlds com,net,dev
```

### Skip Checks

```bash
# Only check domains (skip social media)
chexx0r yungalgo --skip-social

# Only check social media (skip domains)
chexx0r yungalgo --skip-domains
```

### Help

```bash
chexx0r --help
```

## Example Output

```
🔍 Checking availability for: yungalgo
============================================================

📡 DOMAIN AVAILABILITY
------------------------------------------------------------
yungalgo.com                   AVAILABLE
yungalgo.org                   AVAILABLE
yungalgo.io                    AVAILABLE
yungalgo.ai                    AVAILABLE
yungalgo.tech                  AVAILABLE
yungalgo.app                   (error: network timeout)
yungalgo.dev                   AVAILABLE
yungalgo.xyz                   AVAILABLE

📱 SOCIAL MEDIA AVAILABILITY
------------------------------------------------------------
YouTube                        TAKEN
Instagram                      AVAILABLE
TikTok                         AVAILABLE

============================================================
✅ Check complete!
```

## Project Structure

```
chexx0r/
├── Cargo.toml          # Project manifest & dependencies
├── Cargo.lock          # Locked dependency versions
├── BUILD.md            # This file
├── README.md           # Project overview (optional)
├── src/
│   └── main.rs         # Main application code
└── target/             # Build artifacts (generated)
    ├── debug/          # Debug builds
    └── release/        # Release builds
```

## Dependencies

The project uses these key dependencies:

- **clap** (v4.5) - Command-line argument parsing
- **tokio** (v1.42) - Async runtime
- **reqwest** (v0.12) - HTTP client for social media checks
- **domain-check-lib** (v0.6) - Domain availability checking
- **colored** (v2.1) - Terminal colors
- **anyhow** (v1.0) - Error handling
- **futures** (v0.3) - Async utilities

## Homebrew Installation (Future)

To create a Homebrew formula for easy installation:

### Step 1: Create Formula

Create `/usr/local/Homebrew/Library/Taps/homebrew/homebrew-core/Formula/chexx0r.rb`:

```ruby
class Chexx0r < Formula
  desc "CLI tool to check domain and social media username availability"
  homepage "https://github.com/yungalgo/chexx0r"
  url "https://github.com/yungalgo/chexx0r/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "YOUR_SHA256_HERE"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "chexx0r 0.1.0", shell_output("#{bin}/chexx0r --version")
  end
end
```

### Step 2: Publish to GitHub

1. Push your code to GitHub
2. Create a release with a tag (e.g., `v0.1.0`)
3. Update the formula with the correct URL and SHA256

### Step 3: Install via Homebrew

Once published:

```bash
brew tap yungalgo/chexx0r
brew install chexx0r
```

Or submit to official Homebrew core after gaining traction.

## Troubleshooting

### Build Errors

**Problem**: `error: linker 'cc' not found`

**Solution**: Install Xcode Command Line Tools:
```bash
xcode-select --install
```

**Problem**: `error: failed to download dependencies`

**Solution**: Check internet connection or use a different registry mirror:
```bash
# Add to ~/.cargo/config.toml
[source.crates-io]
replace-with = "mirror"

[source.mirror]
registry = "https://github.com/rust-lang/crates.io-index"
```

### Runtime Errors

**Problem**: Network timeouts or connection errors

**Solution**: 
- Check internet connection
- Some platforms (like TikTok) may block automated requests
- Try again with fewer concurrent checks

**Problem**: Domain check errors

**Solution**:
- The domain-check-lib uses RDAP and falls back to WHOIS
- Some TLDs may have rate limits
- Wait a few seconds between checks or reduce concurrency

## Development

### Running Tests

```bash
cargo test
```

### Formatting Code

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Clean Build Artifacts

```bash
cargo clean
```

### Update Dependencies

```bash
cargo update
```

## Customization

### Adding More Social Platforms

Edit `src/main.rs` and add to the `SOCIAL_PLATFORMS` array:

```rust
SocialPlatform {
    name: "Twitter",
    url_template: "https://twitter.com/{}",
    color: Color::Blue,
},
```

### Changing TLD Presets

Edit the `get_preset_tlds()` function in `src/main.rs`:

```rust
fn get_preset_tlds(preset: &str) -> Vec<String> {
    match preset.to_lowercase().as_str() {
        "crypto" => vec!["com", "io", "crypto", "blockchain"],
        // ... add more presets
    }
}
```

## Performance

- **Concurrent checks**: Domain and social media checks run in parallel
- **Timeout**: 10 seconds per check (configurable in code)
- **Typical runtime**: 2-5 seconds for full check

## License

MIT License - see project for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and formatting
5. Submit a pull request

## Resources

- [domain-check GitHub](https://github.com/saidutt46/domain-check)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)

