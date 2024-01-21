{ pkgs ? import <nixpkgs> { } }:

with pkgs;

let
  rustup-toolchain = rust-bin.fromRustupToolchainFile ./rustup-toolchain.toml;
in
mkShell rec {
  packages = [
    cargo-audit
    cargo-edit
    cargo-insta
    cargo-make
    cargo-outdated
    cargo-tarpaulin
    rustup-toolchain
  ];
}
