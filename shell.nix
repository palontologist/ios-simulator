{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    gcc
    gnumake
    cmake
    pkg-config
    git
    nodejs_20
    yarn
    npm
    ruby
    curl
    wget
    jq
    openssl
    unzip
    openssh
    netcat-gnu
  ];

  shellHook = ''
    echo "iOS Development Shell (legacy nix-shell support)"
    echo "For modern flake support, use: nix flake update && nix develop"
  '';
}
