{ pkgs ? import <nixpkgs> { } }:

with pkgs; mkShell {
  nativeBuildInputs = [
    cargo
  ];

  buildInputs = [
    rust-analyzer
    clippy
    rustfmt
  ];

  RUST_BACKTRACE = 1;
}