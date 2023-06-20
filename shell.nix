{ pkgs ? import <nixpkgs> { } }:

with pkgs; mkShell {
  nativeBuildInputs = [
    cargo
  ] ++ (lib.optionals stdenv.isDarwin [
    libiconv
  ]);

  buildInputs = [
    rust-analyzer
    clippy
    rustfmt
  ];

  RUST_BACKTRACE = 1;
}