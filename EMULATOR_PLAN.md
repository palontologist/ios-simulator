# iOSS Emulator - Open Source iOS Simulator

## Project Overview

**iOSS** is an open-source iOS simulator that runs on NixOS without relying on proprietary Apple frameworks or kernels. It provides a testing environment for iOS apps (targeting iOS 14-15) with simulated device features and system frameworks.

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────┐
│         iOSS Emulator Controller                │
│  (Device lifecycle, app management, I/O)        │
└─────────────────┬───────────────────────────────┘
                  │
    ┌─────────────┼─────────────┐
    │             │             │
┌───▼────┐  ┌────▼────┐  ┌────▼────┐
│ Runtime│  │Framework│  │  System  │
│ Engine │  │ Stubs   │  │ Services │
└────┬───┘  └────┬────┘  └────┬────┘
     │            │            │
     └────────────┼────────────┘
                  │
     ┌────────────▼──────────────┐
     │   App Sandbox/Container   │
     │  (ARM64 App Execution)    │
     └───────────────────────────┘
```

### Core Components

1. **Emulator Core** (`emulator/`)
   - Device initialization
   - App lifecycle management
   - System event handling
   - Inter-process communication (IPC)

2. **Runtime Engine** (`emulator/runtime/`)
   - ARM64 app execution sandbox
   - Memory management
   - Thread management
   - Process isolation

3. **Framework Stubs** (`emulator/frameworks/`)
   - UIKit simulation
   - Foundation framework
   - Network/HTTP simulation
   - File system access
   - Camera/Sensor mocks

4. **System Services** (`emulator/services/`)
   - Display/Rendering service
   - Input handling (touch, keyboard)
   - App lifecycle notifications
   - Logging/debugging

5. **Device Simulator** (`emulator/device/`)
   - Device properties (screen, RAM, iOS version)
   - System files and directories
   - Settings/Preferences
   - Device orientation and rotation

## Implementation Phases

### Phase 1: Foundation (Current)
- [x] Project structure and scaffolding
- [ ] Basic emulator control interface (CLI)
- [ ] Device initialization and lifecycle
- [ ] Logging and debugging infrastructure

### Phase 2: Runtime & Execution
- [ ] ARM64 binary execution sandbox
- [ ] Basic app loading and launching
- [ ] Memory allocation and management
- [ ] Simple thread support

### Phase 3: Framework Stubs
- [ ] Core framework implementations
  - Foundation (NSString, NSArray, etc.)
  - UIKit basics (UIViewController, UIView)
  - Network (URLSession stub)
- [ ] File system simulation
- [ ] Settings/User defaults

### Phase 4: System Services
- [ ] Display rendering system
- [ ] Touch input handling
- [ ] App lifecycle notifications
- [ ] System notifications

### Phase 5: Testing & Integration
- [ ] Test harness for iOS apps
- [ ] Integration with Detox
- [ ] Performance optimization
- [ ] Documentation and examples

## Technical Details

### Device Specification

```
Device: iOSS Virtual Device
OS Version: iOS 15.0 (simulated)
Architecture: ARM64
Screen: 390x844 (iPhone 12 equivalent)
RAM: Configurable (default 2GB)
Storage: Virtual filesystem
```

### App Compatibility

- **Supported**: Swift apps, Objective-C apps, React Native, Flutter
- **Limitations**: Real-time features, GPU-intensive graphics, some hardware sensors
- **Not Supported**: Direct Apple frameworks, kernel syscalls, protected APIs

### Communication Protocol

iOSS uses a **protocol-based architecture**:
- Emulator ↔ App: Custom message protocol over sockets
- Emulator ↔ Test Runner: REST API + WebSocket for real-time events

### Storage

```
~/.ioss/
├── devices/
│   └── default/
│       ├── apps/              # Installed apps
│       ├── filesystem/        # Virtual filesystem
│       └── settings.json      # Device settings
└── logs/                      # Emulator logs
```

## Development Stack

- **Language**: Rust (core emulator)
- **Build**: Cargo
- **Testing**: Rust test framework + custom test harness
- **Packaging**: Nix Flake
- **Scripting**: Bash for integration
- **IPC**: Socket-based protocol

## API Design

### Emulator Control API

```bash
# CLI Interface
ioss device create --name test --version 15
ioss device boot
ioss app install <app.ipa>
ioss app launch com.example.app
ioss app test --app com.example.app --test-runner detox
```

### Framework Stubs (Rust API)

```rust
// Example app can call:
UIApplication::shared().present(viewController);
URLSession::shared().dataTask(from: url).resume();
UserDefaults::standard().set(value: 42, forKey: "myKey");
```

## Known Limitations

1. **Graphics**: Simplified rendering, no GPU support
2. **Performance**: Slower than native iOS devices
3. **Hardware**: Limited sensor/camera simulation
4. **APIs**: Only subset of iOS frameworks
5. **Real-time**: May have timing issues with performance-critical code

## Success Metrics

- [x] Can run simple iOS Swift apps
- [ ] Passes Detox E2E tests
- [ ] Supports network requests
- [ ] File system operations work
- [ ] Settings/UserDefaults work
- [ ] App lifecycle events work

## Resources & References

- QEMU documentation (for ARM64 execution concepts)
- iOS Runtime Architecture (reverse engineering resources)
- Frida (for dynamic instrumentation ideas)
- Docker (for containerization ideas)

## Timeline Estimate

- Phase 1: 2-3 days
- Phase 2: 3-5 days
- Phase 3: 4-6 days
- Phase 4: 3-4 days
- Phase 5: 2-3 days

**Total**: 2-3 weeks for MVP

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| ARM64 execution complexity | High | High | Start with simpler bytecode VM, escalate to real execution |
| iOS API compatibility | High | High | Focus on core APIs first, expand gradually |
| Performance issues | Medium | Medium | Optimize hot paths, cache framework calls |
| Binary format changes | Low | Medium | Version management and compatibility layers |

## Next Steps

1. Create Rust project scaffolding
2. Define message protocol between emulator and app
3. Implement basic device lifecycle management
4. Build ARM64 execution sandbox proof-of-concept
5. Create test harness integration
