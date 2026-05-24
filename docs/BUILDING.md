# Building iOS Binaries on NixOS

This guide covers cross-compiling iOS binaries on NixOS using the toolchain provided by this repository.

## Overview

Building iOS binaries on NixOS involves:
- Using LLVM/Clang for ARM64 cross-compilation
- Configuring build environments for iOS SDKs
- Handling code signing and provisioning

## Quick Start

```bash
# Enter build environment
nix develop .#build

# For a React Native app
npm install
npm run build:ios

# Or manually using Xcode tools (if available)
xcodebuild -arch arm64 -sdk iphoneos
```

## LLVM Toolchain

The build environment includes LLVM 16 with clang for cross-compilation:

```bash
# Verify toolchain
clang --version
llvm-config --version

# Check available architectures
clang -print-targets | grep arm64
```

### Supported Architectures

- **arm64**: Default for modern iPhones (iOS 12+)
- **arm64e**: Apple silicon Macs and newer iPhones with PAC
- **x86_64**: iOS simulator (legacy, Intel only)
- **arm64-simulator**: iOS simulator on Apple Silicon Macs

## Build Configuration

### Environment Variables

```bash
# iOS Deployment Target
export IPHONEOS_DEPLOYMENT_TARGET="12.0"

# Architecture selection
export ARCHS="arm64"
export VALID_ARCHS="arm64 arm64e"

# SDK Path (for cross-compilation)
export SDKROOT="$(xcrun --show-sdk-path --sdk iphoneos)"
```

### Build Tools Configuration

Create a `build.nix` file for reproducible builds:

```nix
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    clang_16
    lld
    nodejs_20
    ruby
  ];

  shellHook = ''
    export IPHONEOS_DEPLOYMENT_TARGET="12.0"
    export ARCHS="arm64"
  '';
}
```

## React Native Projects

### Setup

```bash
nix develop .#build

# Install dependencies
npm install
yarn install
pod install --repo-update

# Build for iOS
npm run build:ios
```

### Build Command

```bash
# Development build
npm run ios -- --simulator "iPhone 14"

# Release build
npm run build:ios -- --release
```

## Manual Build with Xcode

If you have Xcode tools installed:

```bash
# List available SDKs
xcrun -sdk iphoneos --show-sdk-path

# Build app
xcodebuild \
  -scheme YourApp \
  -sdk iphoneos \
  -configuration Release \
  -arch arm64 \
  -derivedDataPath build/ \
  CODE_SIGN_IDENTITY="" \
  CODE_SIGNING_REQUIRED=NO
```

## Handling Code Signing

### Development (No Signing)

```bash
# Build without code signing
xcodebuild \
  -scheme YourApp \
  -sdk iphoneos \
  CODE_SIGN_IDENTITY="" \
  CODE_SIGNING_REQUIRED=NO
```

### Production (With Certificates)

On a Mac with Xcode:

```bash
# Export development provisioning profile
security export-identities \
  -p [password] \
  ~/Library/Keychains/login.keychain

# Build with signing
xcodebuild \
  -scheme YourApp \
  -sdk iphoneos \
  -signingIdentity "Developer ID Application"
```

## Output Location

Compiled binaries are typically located at:

```
build/Build/Products/Release-iphoneos/YourApp.app
build/Build/Products/Release-iphoneos/YourApp.app.dSYM
```

## Troubleshooting

### LLVM Not Found

```bash
# Verify LLVM is installed
which clang
clang --version

# Update flake
nix flake update
nix flake check
```

### Compilation Errors

```bash
# Enable verbose output
npm run build:ios -- --verbose

# Check architecture compatibility
file build/Build/Products/Release-iphoneos/YourApp.app/YourApp
# Should output: Mach-O 64-bit executable arm64
```

### Missing SDK Headers

The build environment includes stub SDK files. For full iOS SDK:
- Build on macOS with Xcode
- Use remote Mac via SSH
- Use cloud-based build services

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build iOS

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: cachix/install-nix-action@v20
      - run: nix develop .#build --command npm install
      - run: nix develop .#build --command npm run build:ios
      - uses: actions/upload-artifact@v3
        with:
          name: ios-build
          path: build/Build/Products/Release-iphoneos/
```

## Next Steps

1. [Remote Simulator Guide](./REMOTE_SIMULATOR.md) - Test builds on Mac simulators
2. [E2E Testing Guide](./TESTING.md) - Automate testing with Detox
3. [Setup Guide](./SETUP.md) - Environment configuration options
