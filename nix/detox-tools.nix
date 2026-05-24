{ pkgs }:

with pkgs;

# Detox E2E testing framework for automated iOS/Android testing
# Focuses on functional testing without requiring UI interaction

let
  nodePackages = nodejs_20;
  
in
buildEnv {
  name = "detox-tools";
  paths = [
    # Node.js runtime
    nodePackages
    nodejs_20
    yarn
    npm
    
    # Build tools required for native dependencies
    gcc
    gnumake
    python3
    pkg-config
    
    # Detox CLI will be installed via npm during setup
    # but we provide the runtime dependencies here
    
    # Testing dependencies
    curl
    wget
    
    # Version management
    nvm
    
    # For headless testing bridges
    xvfb-run
    ffmpeg
    
    # Debugging tools
    gdb
    lldb
  ];

  pathsToLink = [ "/bin" "/lib" "/include" "/share" ];
}
