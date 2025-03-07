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
    let kernel = import ./kernel { inherit nixpkgs rust-overlay naersk; };
    in {
      debug = { inherit kernel; };
      devShells.x86_64-linux = {
        kernel = kernel.devShell;
        default = kernel.devShell;
      };
    };
}
