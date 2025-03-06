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
      naersk-lib = pkgs.callPackage naersk { };
      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile
        ./rust-toolchain.toml;
    in {
      pkgs.x86_64-linux = pkgs;
      packages.x86_64-linux.naersk-lib = naersk-lib;
      devShells.x86_64-linux.default = with pkgs;
        mkShell {
          buildInputs = [ rustToolchain pre-commit grub2 ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
    };
}
