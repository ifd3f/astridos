{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, utils, naersk }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ rust-overlay.overlays.default ];
      };
      pkgsCross = pkgs.pkgsCross.x86_64-embedded;
      naersk-lib = pkgs.callPackage naersk { };
      rustToolchain =
        pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in {
      debug.x86_64-linux = { inherit pkgs pkgsCross; };
      packages.x86_64-linux.naersk-lib = naersk-lib;
      devShells.x86_64-linux.default = with pkgs;
        mkShell {
          buildInputs =
            [ rustToolchain pre-commit grub2 pkgsCross.stdenv.cc xorriso ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
    };
}
