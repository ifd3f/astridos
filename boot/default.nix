{ nixpkgs, rust-overlay, naersk }:
let
  pkgs = import nixpkgs {
    system = "x86_64-linux";
    overlays = [ rust-overlay.overlays.default ];
  };
  pkgsCross = pkgs.pkgsCross.x86_64-embedded;
  rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
  naerskLib = pkgs.callPackage naersk {
    cargo = rustToolchain;
    rustc = rustToolchain;
  };
in {
  inherit pkgs pkgsCross naerskLib rustToolchain;

  boot = naerskLib.buildPackage {
    src = ./.;
    cargoBuildOptions = os: os ++ [ "--target" "x86_64-unknown-uefi" ];
  };

  devShell = with pkgs;
    mkShell {
      buildInputs = [ rustToolchain pre-commit pkgsCross.stdenv.cc ];
      RUST_SRC_PATH = rustPlatform.rustLibSrc;
    };
}
