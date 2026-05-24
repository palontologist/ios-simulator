{ pkgs }:

with pkgs;

# iOS development tools and cross-compilation support for NixOS
# Provides tools for building iOS binaries and connecting to remote simulators

let
  clangVersion = "16";
  llvmVersion = "16";
  
  llvm = pkgs."llvm${llvmVersion}";
  clang = pkgs."clang${clangVersion}";
  libcxx = llvm.libcxx;
  
  # iOS SDK sysroot placeholder for remote compilation
  iosSDKStub = runCommand "ios-sdk-stub" {} ''
    mkdir -p $out/usr/include
    mkdir -p $out/usr/lib
    touch $out/usr/include/stdio.h
  '';

in
buildEnv {
  name = "ios-dev-tools";
  paths = [
    # LLVM toolchain for ARM64 cross-compilation
    llvm
    clang
    lld
    libcxx
    libcxxabi
    libunwind
    
    # Build essentials
    gcc
    gnumake
    cmake
    pkg-config
    
    # Code signing and certificate handling
    openssl
    
    # Node-based iOS tools
    nodejs_20
    yarn
    npm
    
    # Ruby for CocoaPods and Fastlane
    ruby
    bundler
    
    # Version control
    git
    
    # Network utilities for remote simulator connection
    openssh
    netcat-gnu
    curl
    
    # Compression and packaging
    unzip
    tar
    gzip
    
    # Development utilities
    jq
    gnused
    gawk
    grep
    coreutils
  ];

  pathsToLink = [ "/bin" "/lib" "/include" "/share" ];
}
