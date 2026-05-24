{
  description = "iOS Development Environment for NixOS";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        iosPlatforms = import ./nix/ios-platforms.nix { inherit pkgs; };
        detoxTools = import ./nix/detox-tools.nix { inherit pkgs; };
        iosDevTools = import ./nix/ios-tools.nix { inherit pkgs; };
        
        # iOSS Emulator build
        iossEmulator = pkgs.rustPlatform.buildRustPackage {
          pname = "ioss-emulator";
          version = "0.1.0";
          src = ./emulator;
          cargoSha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
          doCheck = true;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Core build tools
            gcc
            gnumake
            cmake
            pkg-config
            git

            # Node.js and npm/yarn for React Native and Detox
            nodejs_20
            yarn
            npm

            # iOS cross-compilation tools
            iosDevTools

            # E2E Testing framework
            detoxTools

            # Ruby for CocoaPods (if needed for native dependencies)
            ruby

            # Version management
            nvm
            rbenv

            # Utilities
            curl
            wget
            jq
            openssl
            unzip

            # Development utilities
            vscode
            vim
            tmux

            # For connecting to remote simulators
            openssh
            netcat-gnu
          ];

          shellHook = ''
            echo "iOS Development Environment for NixOS loaded!"
            echo ""
            echo "Available targets:"
            echo "  • E2E Testing: detox --help"
            echo "  • Build Tools: nix flake show"
            echo "  • Remote Simulator: scripts/connect-simulator.sh"
            echo ""
          '';
        };

        devShells.detox = pkgs.mkShell {
          buildInputs = with pkgs; [
            nodejs_20
            yarn
            npm
            detoxTools
          ];
          shellHook = ''
            echo "Detox E2E Testing Environment loaded!"
          '';
        };

        devShells.build = pkgs.mkShell {
          buildInputs = with pkgs; [
            iosDevTools
            nodejs_20
          ];
          shellHook = ''
            echo "iOS Binary Build Environment loaded!"
          '';
        };

        devShells.remote = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssh
            netcat-gnu
            curl
          ];
          shellHook = ''
            echo "Remote Simulator Connection Environment loaded!"
          '';
        };

        devShells.emulator = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            pkg-config
            openssl
          ];
          shellHook = ''
            echo "iOSS Emulator Development Environment loaded!"
            echo "Build: cargo build --release"
            echo "Test: cargo test"
            echo "Install: cargo install --path ."
          '';
        };

        packages = {
          inherit iosDevTools detoxTools iossEmulator;
          default = iosDevTools;
        };

        apps.simulator-proxy = {
          type = "app";
          program = "${./scripts/simulator-proxy.sh}";
        };

        apps.detox-setup = {
          type = "app";
          program = "${./scripts/detox-setup.sh}";
        };
      }
    );
}
