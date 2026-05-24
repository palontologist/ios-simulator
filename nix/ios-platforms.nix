{ pkgs }:

# iOS platform configuration for cross-compilation targets
# Supported architectures and deployment targets

{
  # ARM64 architecture (all modern iPhones)
  arm64 = {
    arch = "arm64";
    triple = "arm64-apple-ios";
    minVersion = "12.0";
    platform = "iphoneos";
  };
  
  # ARM64e (newer iPhones with PAC)
  arm64e = {
    arch = "arm64e";
    triple = "arm64e-apple-ios";
    minVersion = "14.0";
    platform = "iphoneos";
  };

  # Simulator ARM64 (Apple Silicon Mac simulation)
  arm64-simulator = {
    arch = "arm64";
    triple = "arm64-apple-ios-simulator";
    minVersion = "12.0";
    platform = "iphonesimulator";
  };

  # x86_64 simulator (Intel Mac simulation - deprecated but supported)
  x86_64-simulator = {
    arch = "x86_64";
    triple = "x86_64-apple-ios-simulator";
    minVersion = "12.0";
    platform = "iphonesimulator";
  };
}
