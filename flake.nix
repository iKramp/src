{
  description = "Rust OS Kernel Development Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }: 
  let
    system = "x86_64-linux";
    overlays = [ 
      (import rust-overlay)
    ];
    pkgs = import nixpkgs { inherit system overlays; };

    rust = pkgs.rust-bin.stable.latest.default;

  in {
    devShells.${system}.default = pkgs.mkShell rec {
      buildInputs = [
        rust
        pkgs.rust-analyzer
        pkgs.clippy
      ];

      shellHook = ''
        exec zsh -c "nvim"
      '';
    };
  };
}

