# Connecting to Remote iOS Simulators on macOS

This guide covers setting up connections to iOS simulators running on macOS machines from your NixOS development environment.

## Overview

By connecting to a remote macOS machine with Xcode and iOS simulators, you can:
- Run automated E2E tests from NixOS CI/CD pipelines
- Test builds on actual iOS simulator environments
- Avoid requiring macOS for local development
- Scale testing across multiple Mac machines

## Prerequisites

### On macOS Host

1. **Xcode installed** with iOS SDK
2. **SSH enabled** for remote access
3. **Network connectivity** from NixOS to macOS

### On NixOS

1. **SSH client** configured with key authentication
2. **Network tools** (nc, curl) for connectivity testing

## Setup

### Step 1: Enable SSH on macOS

```bash
# On macOS
System Preferences > Sharing > Remote Login

# Or via terminal
sudo systemsetup -setremotelogin on

# Verify SSH is running
sudo launchctl list | grep ssh
```

### Step 2: Configure SSH Keys

```bash
# On NixOS - generate SSH key if needed
ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519 -N ""

# Copy public key to macOS
ssh-copy-id -i ~/.ssh/id_ed25519.pub username@mac-host.local

# Test connection
ssh username@mac-host.local "echo OK"
```

### Step 3: Set Environment Variables

```bash
# Set on NixOS development machine
export MAC_HOST="mac-mini.local"      # macOS hostname or IP
export MAC_USER="developer"           # SSH username
export SIMULATOR_PORT="5037"          # Default simulator port
```

### Step 4: Create SSH Tunnel

```bash
# Enter remote environment
nix develop .#remote

# Connect to simulator
./scripts/connect-simulator.sh
```

The script will:
- Test SSH connectivity
- Create port forwarding tunnel
- Verify connection
- Keep connection alive

## SSH Tunnel Details

The connection script establishes three port forwards:

```
Local 5037  → Remote 5037  (Simulator daemon)
Local 5038  → Remote 5038  (XCTestDaemon)
Local 5039  → Remote 5039  (Xcode services)
```

### Manual SSH Tunnel Setup

```bash
# Create SSH tunnel manually
ssh -N \
    -L 5037:localhost:5037 \
    -L 5038:localhost:5038 \
    -L 5039:localhost:5039 \
    username@mac-host.local &

# Keep in background, get process ID
echo $! > ~/.ssh/simulator-tunnel.pid
```

## Using Remote Simulators

### Running E2E Tests

```bash
# Terminal 1: Establish remote connection
nix develop .#remote
./scripts/connect-simulator.sh

# Terminal 2: Run Detox tests
nix develop .#detox
detox test e2e --configuration ios.sim.release --cleanup
```

### Building on NixOS, Testing on Remote

```bash
# On NixOS - Build the app
nix develop .#build
npm run build:ios

# Establish simulator connection
nix develop .#remote
./scripts/connect-simulator.sh

# In another terminal - Test the build
nix develop .#detox
detox build e2e --configuration ios.sim.release
detox test e2e --configuration ios.sim.release
```

### Continuous Connection

```bash
# Create persistent SSH session
screen -S simulator-tunnel
ssh -N -L 5037:localhost:5037 \
    -L 5038:localhost:5038 \
    -L 5039:localhost:5039 \
    username@mac-host.local

# Detach: Ctrl+A then D
# Reattach: screen -r simulator-tunnel
# Kill: screen -X -S simulator-tunnel quit
```

## Detox Configuration for Remote

Update `.detoxrc.json` to use remote simulator:

```json
{
  "configurations": {
    "ios.sim.release": {
      "device": {
        "type": "iPhone 14",
        "host": "localhost:5037"
      },
      "app": "ios.release"
    }
  },
  "apps": {
    "ios.release": {
      "type": "ios.app",
      "binaryPath": "ios/build/Build/Products/Release-iphonesimulator/YourApp.app",
      "build": "xcodebuild -workspace ios/YourApp.xcworkspace -scheme YourApp -configuration Release -sdk iphonesimulator -derivedDataPath ios/build"
    }
  }
}
```

## CI/CD Integration

### GitHub Actions with Remote Simulator

```yaml
name: Remote E2E Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    env:
      MAC_HOST: ${{ secrets.MAC_HOST }}
      MAC_USER: ${{ secrets.MAC_USER }}
      SSH_PRIVATE_KEY: ${{ secrets.SSH_PRIVATE_KEY }}
    
    steps:
      - uses: actions/checkout@v3
      - uses: cachix/install-nix-action@v20
      
      - name: Setup SSH
        run: |
          mkdir -p ~/.ssh
          echo "$SSH_PRIVATE_KEY" > ~/.ssh/id_ed25519
          chmod 600 ~/.ssh/id_ed25519
          ssh-keyscan -H $MAC_HOST >> ~/.ssh/known_hosts
      
      - name: Connect to Remote Simulator
        run: nix develop .#remote --command ./scripts/connect-simulator.sh &
      
      - name: Wait for Connection
        run: sleep 5
      
      - name: Build App
        run: nix develop .#build --command npm run build:ios
      
      - name: Run Tests
        run: nix develop .#detox --command ./scripts/detox-setup.sh && detox test e2e --configuration ios.sim.release
```

Add secrets to GitHub repository:
- `MAC_HOST`: Hostname or IP of macOS machine
- `MAC_USER`: SSH username
- `SSH_PRIVATE_KEY`: Private SSH key

## Simulator Management on macOS

### List Available Simulators

```bash
# On macOS
xcrun simctl list devices

# Via SSH from NixOS
ssh username@mac-host.local "xcrun simctl list devices"
```

### Launch Simulator

```bash
# On macOS
xcrun simctl boot "iPhone 14"
open /Applications/Simulator.app

# Via SSH
ssh username@mac-host.local "xcrun simctl boot 'iPhone 14'"
```

### Install App on Simulator

```bash
# On macOS
xcrun simctl install booted path/to/App.app

# Via SSH
ssh username@mac-host.local "xcrun simctl install booted /path/to/App.app"
```

## Troubleshooting

### SSH Connection Refused

```bash
# Verify SSH is running on macOS
ssh -v username@mac-host.local

# Enable SSH on macOS
sudo systemsetup -setremotelogin on

# Check firewall
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate
```

### Port Already in Use

```bash
# Check ports on NixOS
lsof -i :5037

# Kill existing connections
pkill -f "ssh.*mac-host"
sleep 2

# Or use different ports
export SIMULATOR_PORT=6037
./scripts/connect-simulator.sh
```

### Timeout Issues

```bash
# Increase SSH timeout
ssh -o ConnectTimeout=30 -o ServerAliveInterval=60 \
    username@mac-host.local

# Edit ~/.ssh/config
Host mac-host.local
    User developer
    ConnectTimeout 30
    ServerAliveInterval 60
    ServerAliveCountMax 10
```

### Authentication Issues

```bash
# Verify key permissions
chmod 600 ~/.ssh/id_ed25519
chmod 644 ~/.ssh/id_ed25519.pub

# Add key to SSH agent
ssh-add ~/.ssh/id_ed25519

# Verify on macOS
cat ~/.ssh/authorized_keys | grep "id_ed25519"
```

### Network Issues

```bash
# Test connectivity
ping mac-host.local
nc -zv mac-host.local 22

# Check network routing
route -n

# DNS resolution
nslookup mac-host.local
```

## Advanced Configuration

### Multiple Mac Machines

Create configuration for multiple Macs:

```bash
# Connect to first Mac
export MAC_HOST="mac-1.local"
export MAC_USER="dev1"
export LOCAL_PORT=5037
./scripts/connect-simulator.sh

# In another terminal, connect to second Mac
export MAC_HOST="mac-2.local"
export MAC_USER="dev2"
export LOCAL_PORT=6037
./scripts/connect-simulator.sh
```

### Load Balancing

Use HAProxy to distribute tests across multiple Macs:

```bash
# Install HAProxy
nix-shell -p haproxy

# Create haproxy.cfg
global
  daemon
  log localhost local0

defaults
  balance roundrobin
  timeout connect 5000
  timeout client 50000
  timeout server 50000

frontend simulator_in
  bind localhost:5037
  default_backend simulators

backend simulators
  server mac1 mac-1.local:5037 check
  server mac2 mac-2.local:5037 check
```

## Next Steps

1. [Building Guide](./BUILDING.md) - Build iOS binaries
2. [Testing Guide](./TESTING.md) - Run E2E tests
3. [Setup Guide](./SETUP.md) - Environment configuration

## Resources

- [macOS SSH Configuration](https://support.apple.com/en-us/HT201926)
- [Xcode Simulator Documentation](https://developer.apple.com/library/archive/documentation/IDEs/Conceptual/iOS_Simulator_Guide/)
- [Detox Remote Testing](https://wix.github.io/Detox/docs/guide/running-on-a-cloud-machine)
