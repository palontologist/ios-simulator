# iOS Development Environment Setup for NixOS

This guide walks you through setting up a complete iOS development environment on NixOS, including tools for building iOS binaries, running automated E2E tests, and connecting to remote simulators.

## Prerequisites

- NixOS or Nix package manager installed
- For remote simulator connection: macOS host with Xcode installed
- Git for version control

## Quick Start

### Using Flakes (Recommended)

```bash
# Enter development environment
nix flake update
nix develop

# Or enter a specific shell
nix develop .#detox     # For E2E testing
nix develop .#build     # For building binaries
nix develop .#remote    # For remote simulator connection
```

### Using Legacy Shell

```bash
nix-shell shell.nix
```

## Project Structure

```
.
├── flake.nix                      # Nix flake configuration
├── shell.nix                      # Legacy shell.nix
├── nix/
│   ├── ios-tools.nix             # iOS development tools
│   ├── detox-tools.nix           # Detox E2E testing
│   └── ios-platforms.nix         # Platform configurations
├── scripts/
│   ├── connect-simulator.sh       # Remote simulator connection
│   ├── detox-setup.sh            # Detox framework setup
│   └── simulator-proxy.sh        # Functional testing bridge
└── docs/
    ├── SETUP.md                  # This file
    ├── BUILDING.md               # Building iOS binaries
    ├── TESTING.md                # E2E testing guide
    └── REMOTE_SIMULATOR.md       # Remote simulator guide
```

## Development Environments

### 1. Default Environment (All Tools)

```bash
nix develop
```

Includes all iOS development tools, Detox, build tools, and remote simulator utilities.

### 2. E2E Testing (Detox)

```bash
nix develop .#detox
```

Optimized for running automated E2E tests with Detox.

### 3. Binary Building

```bash
nix develop .#build
```

Tools for cross-compiling iOS binaries on NixOS.

### 4. Remote Simulator Connection

```bash
nix develop .#remote
```

SSH and networking tools for connecting to macOS simulators.

## Building iOS Binaries

See [BUILDING.md](./BUILDING.md) for detailed instructions on cross-compiling iOS applications on NixOS.

### Quick Example

```bash
# Enter build environment
nix develop .#build

# Your build commands here
npm install
npm run build:ios
```

## E2E Testing with Detox

See [TESTING.md](./TESTING.md) for comprehensive E2E testing setup and usage.

### Quick Setup

```bash
nix develop .#detox
./scripts/detox-setup.sh
detox test e2e --configuration ios.sim.release --cleanup
```

## Connecting to Remote Simulators

### Setup

1. **Prepare macOS host:**
   ```bash
   # On macOS
   System Preferences > Sharing > Remote Login (enable SSH)
   ```

2. **Configure connection:**
   ```bash
   export MAC_HOST="your-mac-host.local"
   export MAC_USER="your-username"
   ```

3. **Connect:**
   ```bash
   nix develop .#remote
   ./scripts/connect-simulator.sh
   ```

See [REMOTE_SIMULATOR.md](./REMOTE_SIMULATOR.md) for detailed instructions.

## Simulator Proxy (Functional Testing Bridge)

For functional testing without full UI emulation:

```bash
./scripts/simulator-proxy.sh start
curl http://localhost:8080/simulator/status
./scripts/simulator-proxy.sh validate-app
```

## Available Tools

The environment includes:

- **Build Tools**: gcc, make, cmake, pkg-config
- **Node.js**: v20 LTS with npm and yarn
- **LLVM/Clang**: For ARM64 cross-compilation
- **Ruby**: For CocoaPods and Fastlane
- **Git**: Version control
- **SSH/NetCat**: Remote connections
- **Testing**: Detox framework and CLI

## Environment Variables

Common environment variables:

```bash
# Remote simulator connection
export MAC_HOST="mac-mini.local"
export MAC_USER="developer"
export SIMULATOR_PORT="5037"

# Proxy configuration
export PROXY_PORT="8080"

# Build configuration
export IPHONEOS_DEPLOYMENT_TARGET="12.0"
export ARCHS="arm64"
```

## Troubleshooting

### SSH Connection Issues

```bash
# Test SSH connectivity
ssh -v user@mac-host.local

# Enable SSH on macOS if needed
sudo systemsetup -setremotelogin on
```

### Port Forwarding Issues

```bash
# Check if ports are in use
lsof -i :5037

# Kill existing connections
pkill -f "ssh.*mac-host"
```

### Detox Installation Issues

```bash
# Clear npm cache
npm cache clean --force

# Reinstall detox
npm install --save-dev detox-cli detox
```

### LLVM/Clang Issues

```bash
# Verify LLVM installation
clang --version
llvm-config --version

# Try updating nixpkgs
nix flake update
```

## Next Steps

1. Read [BUILDING.md](./BUILDING.md) to start building iOS binaries
2. Follow [TESTING.md](./TESTING.md) for E2E testing setup
3. Check [REMOTE_SIMULATOR.md](./REMOTE_SIMULATOR.md) for remote simulator connection

## Resources

- [Nix Language Reference](https://nixos.wiki/wiki/Nix_Language)
- [Flakes Documentation](https://nixos.wiki/wiki/Flakes)
- [Detox Documentation](https://wix.github.io/Detox)
- [Xcode Simulator Guide](https://developer.apple.com/library/archive/documentation/IDEs/Conceptual/iOS_Simulator_Guide/)

## Support

For issues or questions:
- Check existing documentation in `docs/`
- Review error messages carefully
- Enable verbose logging: `VERBOSE=true script.sh`
