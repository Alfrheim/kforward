{
  description = "A devShell example using rust-overlay";

  # https://github.com/oxalica/rust-overlay

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [
              openssl
              pkg-config
              eza
              fd
              rust-analyzer
              cargo-feature

              # set here the rust version you want to use
              # rust-bin.stable.latest.default # Stable rust, default profile. If not sure, always choose this.
              # rust-bin.beta.latest.default   # Wanna test beta compiler.
              # rust-bin.stable.latest.minimal # I don't need anything other than rustc, cargo, rust-std. Bye rustfmt, clippy, etc.
              # rust-bin.beta.latest.minimal

              # rust-bin.stable."1.48.0".default
              # rust-bin.beta."2021-01-01".default
              # rust-bin.nightly."2020-12-31".default

              rust-bin.beta.latest.default
            ];

            shellHook = ''
              alias ls=eza
              alias find=fd
            '';
          };
        }
    );
}
