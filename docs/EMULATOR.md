# iOSS Emulator Integration Guide

## Overview

iOSS is an open-source iOS emulator written in Rust that provides a lightweight iOS 14/15 simulation environment on NixOS. This guide covers how to build and use it.

## Installation via Nix Flake

The emulator is integrated into the main flake and can be built with:

```bash
# Build the emulator
nix build .#ioss

# Or enter the emulator development environment
nix develop .#emulator

# Build the CLI binary
cargo build --release
```

## Quick Start

### Create a Virtual Device

```bash
ioss device create --name "My iPhone" --ios 15
```

### List Devices

```bash
ioss device list
```

### Boot a Device

```bash
ioss device boot <device-id>
```

### View Device Info

```bash
ioss device info <device-id>
```

## Architecture

### Core Components

1. **Device Manager** - Creates and manages virtual iOS devices
2. **Runtime Engine** - Executes app binaries in a sandbox
3. **Framework Stubs** - Simulates iOS frameworks (UIKit, Foundation, etc.)
4. **System Services** - Handles display, input, notifications
5. **CLI Interface** - Command-line control of emulator

### Data Model

Devices are stored in `~/.ioss/devices/` with the following structure:

```
~/.ioss/devices/{device-id}/
├── apps/                          # Installed apps
├── filesystem/                    # Virtual filesystem
│   ├── Documents/
│   ├── Library/
│   └── tmp/
└── settings.json                 # Device configuration
```

## Development

### Building from Source

```bash
cd emulator
cargo build --release

# Run tests
cargo test

# Install binary
cargo install --path .
```

### Project Structure

```
emulator/
├── src/
│   ├── lib.rs                   # Library root
│   ├── bin/main.rs              # CLI binary
│   ├── error.rs                 # Error types
│   ├── device.rs                # Device simulation
│   ├── emulator.rs              # Emulator controller
│   ├── frameworks/              # Framework stubs
│   │   ├── foundation.rs
│   │   ├── uikit.rs
│   │   └── network.rs
│   ├── runtime/                 # Execution runtime
│   │   └── mod.rs
│   └── services/                # System services
│       └── mod.rs
├── Cargo.toml
└── Cargo.lock
```

## API Reference

### Device Management

```rust
// Create emulator instance
let mut emulator = Emulator::new(None)?;

// Create device
let device_id = emulator.create_device(
    "iPhone 12".to_string(),
    iOSVersion::iOS15
)?;

// Boot device
let device = emulator.get_device_mut(&device_id)?;
device.boot().await?;

// Install app
device.install_app(&Path::new("app.ipa"), "com.example.app").await?;
```

## CLI Commands

### Device Commands

```bash
ioss device create --name "iPhone 12" --ios 15
ioss device list
ioss device boot <device-id>
ioss device shutdown <device-id>
ioss device delete <device-id>
ioss device info <device-id>
```

### App Commands (Coming Soon)

```bash
ioss app install --device <id> --path app.ipa --app-id com.example.app
ioss app launch --device <id> com.example.app
ioss app list --device <id>
```

### Simulator Commands

```bash
ioss simulator info
ioss simulator status
```

## Limitations (Phase 1)

This is an MVP (Minimum Viable Product). Known limitations:

- ❌ No actual app execution (Phase 2)
- ❌ No graphics/rendering (Phase 4)
- ❌ No networking (Phase 3)
- ❌ Limited framework support (Phase 3)
- ❌ No sensor simulation (Phase 4+)
- ✅ Device lifecycle management
- ✅ Virtual filesystem
- ✅ Device configuration

## Roadmap

- **Phase 1** ✅ Foundation (current)
  - Device creation and lifecycle
  - Virtual filesystem
  - CLI interface

- **Phase 2** 🚧 Runtime & Execution
  - ARM64 app execution
  - Process management
  - Memory management

- **Phase 3** ⏳ Framework Stubs
  - Foundation framework
  - UIKit basics
  - Network simulation

- **Phase 4** ⏳ System Services
  - Display rendering
  - Input handling
  - App lifecycle

- **Phase 5** ⏳ Testing & Integration
  - Detox integration
  - Performance optimization
  - Full documentation

## Integration with Detox

Once Phase 2-3 are complete, the emulator will integrate with Detox:

```bash
# Start emulator
ioss device boot <device-id>
ioss app install --device <device-id> --path app.ipa --app-id com.example.app

# Run Detox tests
detox test e2e --configuration ios.ioss
```

## Troubleshooting

### Build Errors

```bash
# Update dependencies
cargo update

# Clean build
cargo clean
cargo build
```

### Device Creation Fails

```bash
# Check permissions
ls -la ~/.ioss/

# Clear and retry
rm -rf ~/.ioss/
ioss device create --name "Test" --ios 15
```

## Contributing

Contributions are welcome! Areas needing work:

- [ ] ARM64 execution engine
- [ ] Graphics/rendering system
- [ ] Network simulation
- [ ] More framework stubs
- [ ] Performance optimization
- [ ] Documentation

See `EMULATOR_PLAN.md` for detailed architecture and roadmap.

## License

MIT - See LICENSE file

## Resources

- Rust Book: https://doc.rust-lang.org/book/
- Cargo Guide: https://doc.rust-lang.org/cargo/
- iOS Runtime Architecture: https://developer.apple.com/library/archive/documentation/
- QEMU/ARM64: https://www.qemu.org/
