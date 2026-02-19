{ self, pkgs, ... }:
pkgs.mkShell {
  packages = with pkgs; [
    # Rust
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    cargo-watch

    # Nix
    alejandra
    nixd
    deadnix
    statix
    self.formatter.${system}
  ];
}
