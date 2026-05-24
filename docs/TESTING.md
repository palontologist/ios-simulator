# E2E Testing with Detox on NixOS

This guide covers setting up and running automated E2E tests for iOS apps using Detox on NixOS.

## Overview

Detox is a framework for automated testing of mobile apps. It focuses on functionality testing rather than UI interaction, making it ideal for NixOS-based CI/CD pipelines.

## Setup

### Initial Installation

```bash
# Enter Detox environment
nix develop .#detox

# Run setup script
./scripts/detox-setup.sh
```

This will:
- Install Node.js dependencies
- Create example Detox configuration
- Build native dependencies
- Set up testing framework

### Configuration

Edit `.detoxrc.json` to configure your app:

```json
{
  "configurations": {
    "ios.sim.release": {
      "device": {
        "type": "iPhone 14"
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
  },
  "testRunner": "jest"
}
```

## Writing Tests

Create test files in the `e2e/` directory:

### Basic Test Example

```javascript
// e2e/firstTest.e2e.js
describe('Login Flow', () => {
  beforeAll(async () => {
    await device.launchApp();
  });

  beforeEach(async () => {
    await device.reloadReactNative();
  });

  it('should show welcome screen', async () => {
    await expect(element(by.text('Welcome'))).toBeVisible();
  });

  it('should login with valid credentials', async () => {
    await element(by.id('email')).typeText('user@example.com');
    await element(by.id('password')).typeText('password123');
    await element(by.text('Login')).multiTap();
    
    await waitFor(element(by.text('Home')))
      .toBeVisible()
      .withTimeout(5000);
  });

  it('should reject invalid credentials', async () => {
    await element(by.id('email')).typeText('invalid@example.com');
    await element(by.id('password')).typeText('wrongpassword');
    await element(by.text('Login')).multiTap();
    
    await expect(element(by.text('Invalid credentials')))
      .toBeVisible();
  });
});
```

### Testing Patterns

#### Finding Elements

```javascript
// By text
by.text('Button Text')

// By ID
by.id('uniqueId')

// By type
by.type('RCTImageView')

// Complex selectors
by.id('parent').and(by.text('Child'))
```

#### Interactions

```javascript
// Tap
await element(by.text('Tap me')).multiTap();

// Type text
await element(by.id('input')).typeText('Hello');

// Scroll
await waitFor(element(by.text('Scrolled item')))
  .toBeVisible()
  .whileElement(by.id('list'))
  .scroll(200, 'down');

// Swipe
await element(by.id('swipable')).multiSwipe();
```

#### Assertions

```javascript
// Visibility
await expect(element(by.id('visible'))).toBeVisible();

// Text matching
await expect(element(by.id('label'))).toHaveText('Expected Text');

// Value
await expect(element(by.id('input'))).toHaveValue('input value');

// Not visible
await expect(element(by.id('hidden'))).not.toBeVisible();
```

## Running Tests

### Local Testing with Remote Simulator

```bash
# Connect to remote Mac simulator
export MAC_HOST="your-mac.local"
export MAC_USER="username"
./scripts/connect-simulator.sh

# In another terminal, run tests
nix develop .#detox

# Build for testing
detox build-framework-cache
detox build e2e --configuration ios.sim.release

# Run tests
detox test e2e --configuration ios.sim.release --cleanup

# With specific test file
detox test e2e/firstTest.e2e.js --configuration ios.sim.release
```

### Headless Testing (CI/CD)

```bash
# Run tests without UI
detox test e2e \
  --configuration ios.sim.release \
  --cleanup \
  --record-logs all \
  --take-screenshots all
```

### Test Reports

```bash
# Generate artifacts directory
mkdir -p artifacts

# Run tests with artifacts
detox test e2e \
  --configuration ios.sim.release \
  --record-logs all \
  --record-artifacts all \
  --artifact-location artifacts
```

## Functional Testing Bridge

For testing without simulator GUI:

```bash
# Start proxy server
./scripts/simulator-proxy.sh start

# Validate app functionality
./scripts/simulator-proxy.sh validate-app com.myapp

# Check status
./scripts/simulator-proxy.sh status
```

The proxy provides REST endpoints:
- `GET /simulator/status` - Proxy status
- `GET /simulator/devices` - Available devices
- `POST /simulator/validate-app` - Validate app functionality
- `GET /health` - Health check

## CI/CD Integration

### GitHub Actions Workflow

```yaml
name: E2E Tests

on: [push, pull_request]

jobs:
  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: cachix/install-nix-action@v20
      
      - name: Build App
        run: nix develop .#build --command npm run build:ios
      
      - name: Install Dependencies
        run: nix develop .#detox --command npm install
      
      - name: Setup Detox
        run: nix develop .#detox --command ./scripts/detox-setup.sh
      
      - name: Build Detox
        run: nix develop .#detox --command detox build e2e --configuration ios.sim.release
      
      - name: Run Tests
        run: nix develop .#detox --command detox test e2e --configuration ios.sim.release --cleanup
      
      - uses: actions/upload-artifact@v3
        if: failure()
        with:
          name: detox-artifacts
          path: artifacts/
```

## Troubleshooting

### Build Errors

```bash
# Clear build cache
rm -rf artifacts/
npm cache clean --force

# Rebuild
detox build-framework-cache
detox build e2e --configuration ios.sim.release
```

### Connection Issues

```bash
# Verify remote simulator connection
./scripts/connect-simulator.sh

# Test proxy connectivity
curl http://localhost:8080/health
```

### Test Timeouts

Increase timeout in tests:

```javascript
await waitFor(element(by.id('element')))
  .toBeVisible()
  .withTimeout(10000); // 10 seconds
```

### Memory Issues

```bash
# Limit parallel tests
detox test e2e --configuration ios.sim.release --maxWorkers 1
```

## Advanced Configuration

### Custom Test Runner

```json
{
  "testRunner": "jest",
  "configs": {
    "jestOptions": {
      "preset": "react-native",
      "testTimeout": 120000,
      "maxWorkers": 2
    }
  }
}
```

### Device Specifications

```json
{
  "configurations": {
    "ios.sim.custom": {
      "device": {
        "type": "iPhone 14",
        "os": "iOS",
        "osVersion": "16.0"
      },
      "app": "ios.release"
    }
  }
}
```

## Next Steps

1. [Building Guide](./BUILDING.md) - Build iOS binaries
2. [Setup Guide](./SETUP.md) - Environment configuration
3. [Remote Simulator Guide](./REMOTE_SIMULATOR.md) - Connect to Mac simulators

## Resources

- [Detox API Reference](https://wix.github.io/Detox/docs/getting-started/getting-started)
- [Jest Testing Framework](https://jestjs.io/)
- [React Native Testing Guide](https://reactnative.dev/docs/testing-overview)
