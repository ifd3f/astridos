{ nixpkgs, rust-overlay, naersk }:
let
  pkgs = import nixpkgs {
    system = "x86_64-linux";
    overlays = [ rust-overlay.overlays.default ];
  };
  pkgsCross = pkgs.pkgsCross.x86_64-embedded;
  naersk-lib = pkgs.callPackage naersk { };
  rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in {
  inherit pkgs pkgsCross naersk-lib rustToolchain;

  devShell = with pkgs;
    mkShell {
      buildInputs =
        [ rustToolchain pre-commit grub2 pkgsCross.stdenv.cc xorriso ];
      RUST_SRC_PATH = rustPlatform.rustLibSrc;
    };
}
