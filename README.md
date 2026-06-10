# iOS Development Environment for NixOS

A comprehensive NixOS Flake for iOS app development, providing tools for:

- **Building iOS Binaries**: Cross-compile iOS apps on NixOS using LLVM/Clang
- **E2E Testing**: Run automated tests with Detox framework
- **Remote Simulators**: Connect to macOS simulators over SSH
- **Functional Testing Bridge**: Test app logic without full UI emulation
 Quick Start

```bash
# Enter default development environment
nix flake update
nix develop

# Or enter a specific shell
nix develop .#detox     # E2E testing with Detox
nix develop .#build     # iOS binary building
nix develop .#remote    # Remote simulator connection
```

## What's Included

### Build Environment
- LLVM/Clang 16 for ARM64 cross-compilation
- Node.js 20 LTS
- Ruby with CocoaPods and Fastlane support
- CMake, Make, and standard build tools

### Testing Environment (Detox)
- Detox E2E testing framework
- Jest test runner
- ffmpeg and xvfb for headless testing
- Python and debugging tools (gdb, lldb)

### Remote Simulator Tools
- SSH client for remote connections
- NetCat for connectivity testing
- Port forwarding utilities

### Utilities
- Git for version control
- jq for JSON processing
- Compression tools (zip, gzip, tar)

## Project Structure

```
.
├── flake.nix                    # Nix flake configuration
├── shell.nix                    # Legacy shell.nix support
├── nix/
│   ├── ios-tools.nix           # iOS development tools
│   ├── detox-tools.nix         # Detox testing framework
│   └── ios-platforms.nix       # iOS platform configurations
├── scripts/
│   ├── connect-simulator.sh     # Remote simulator SSH tunnel
│   ├── detox-setup.sh          # Detox initialization
│   └── simulator-proxy.sh      # Functional testing proxy
└── docs/
    ├── SETUP.md                # Setup guide
    ├── BUILDING.md             # Building binaries guide
    ├── TESTING.md              # E2E testing guide
    └── REMOTE_SIMULATOR.md     # Remote simulator guide
```

## Core Goals

### 1. Run Automated E2E Tests (Detox)
Focus on functionality testing without requiring interactive UI simulation:

```bash
nix develop .#detox
./scripts/detox-setup.sh
detox test e2e --configuration ios.sim.release
```

### 2. Build iOS Binaries
Cross-compile iOS apps on NixOS for ARM64 architecture:

```bash
nix develop .#build
npm run build:ios
```

### 3. Connect to Remote Simulators
Test builds on macOS simulators over SSH:

```bash
export MAC_HOST="your-mac.local"
export MAC_USER="username"
nix develop .#remote
./scripts/connect-simulator.sh
```

### 4. Simulator Proxy (Functional Testing Bridge)
Validate app features without full UI emulation:

```bash
./scripts/simulator-proxy.sh start
./scripts/simulator-proxy.sh validate-app com.myapp
```

## Documentation

- **[SETUP.md](./docs/SETUP.md)** - Initial environment setup and configuration
- **[BUILDING.md](./docs/BUILDING.md)** - Cross-compiling iOS binaries on NixOS
- **[TESTING.md](./docs/TESTING.md)** - E2E testing with Detox framework
- **[REMOTE_SIMULATOR.md](./docs/REMOTE_SIMULATOR.md)** - Connecting to macOS simulators

## Usage Examples

### Build and Test Workflow

```bash
# Build the app
nix develop .#build
npm install
npm run build:ios

# Connect to remote simulator
nix develop .#remote
./scripts/connect-simulator.sh &

# Run tests
nix develop .#detox
npm run detox:build
npm run detox:test
```

### CI/CD Pipeline

GitHub Actions workflow for automated testing:

```yaml
name: iOS Build & Test

on: [push, pull_request]

jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: cachix/install-nix-action@v20
      
      - name: Build iOS App
        run: nix develop .#build --command npm run build:ios
      
      - name: Run E2E Tests
        run: nix develop .#detox --command ./scripts/detox-setup.sh && detox test e2e
```

### Development Environment

```bash
# Enter the full development shell
nix develop

# All tools available for development
npm install
bundle install

# Build app
npm run build:ios

# Run tests
npm run test:e2e
```

## Environment Variables

```bash
# Remote simulator connection
export MAC_HOST="mac-mini.local"
export MAC_USER="developer"
export SIMULATOR_PORT="5037"

# Build configuration
export IPHONEOS_DEPLOYMENT_TARGET="12.0"
export ARCHS="arm64"

# Proxy server
export PROXY_PORT="8080"
```

## Troubleshooting

### SSH Connection Issues
```bash
ssh -v user@mac-host.local
# Enable SSH: System Preferences > Sharing > Remote Login
```

### Port Already in Use
```bash
pkill -f "ssh.*mac-host"
lsof -i :5037
```

### LLVM/Clang Not Found
```bash
nix flake update
clang --version
```

See detailed troubleshooting in individual guide files.

## License

MIT License - See LICENSE file

## Contributing

Contributions welcome! Please ensure:
- Nix code follows [nixpkgs conventions](https://github.com/nixos/nixpkgs)
- Bash scripts are POSIX-compliant
- Documentation is up-to-date
- Changes are tested locally

## Resources

- [Nix Flakes Documentation](https://nixos.wiki/wiki/Flakes)
- [Detox Documentation](https://wix.github.io/Detox/)
- [iOS Development with React Native](https://reactnative.dev/)
- [Xcode Simulator Guide](https://developer.apple.com/library/archive/documentation/IDEs/Conceptual/iOS_Simulator_Guide/)

## Support

For issues or questions:
1. Check the relevant documentation file
2. Review error messages carefully  
3. Enable verbose logging: `VERBOSE=true script.sh`
4. Check [GitHub Issues](https://github.com/palontologist/ios-simulator/issues)
