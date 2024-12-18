{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { 
          inherit system overlays;
#          overlays = [
#            (final: prev: {
#              anchor = prev.anchor.overrideAttrs (_: rec {
#                version = "0.30.0";
#                src = prev.fetchFromGitHub {
#                  owner = "coral-xyz";
#                  repo = "anchor";
#                  rev = "v${version}";
#                  hash = "sha256-RvYJoGmACoPfdbeXK2uYutleHPT0kAlOCDJX4NP9q4I=";
#                  fetchSubmodules = true;
#                };
#              });
#            })
#          ];
        };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [ 
            pkg-config
            libtool
            automake
            clang
            openssl
            autoconf
            libudev-zero
            pre-commit
            pnpm
            solana-cli
            nodejs_22
            anchor
            rust-bin.stable."1.79.0".default
          ];
        };
      }
    );
}
