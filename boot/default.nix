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
in rec {
  inherit pkgs pkgsCross naerskLib rustToolchain;

  boot = naerskLib.buildPackage {
    src = ./.;
    cargoBuildOptions = os: os ++ [ "--target" "x86_64-unknown-uefi" ];
  };

  esp = pkgs.runCommand "astridos-bootos-testesp" {
    buildInputs = with pkgs; [ mtools ];
  } ''
    mkdir -p $out/efi/boot
    # dd if=/dev/zero of=$out/esp.img bs=512 count=91669
    # mformat -i $out/esp.img -h 32 -t 32 -n 64 -c 1
    # mcopy -i $out/esp.img ${boot}/bin/astridos-bootos.efi ::
    cp ${boot}/bin/astridos-bootos.efi $out/efi/boot/bootx64.efi
  '';

  ovmf = pkgs.OVMF.fd;

  testesp = let ovmf = pkgs.OVMF.fd;
  in pkgs.writeShellScriptBin "run-astridos-bootos" ''
    cp -r ${esp} testesp
    chmod -R +w testesp
    ${pkgs.qemu}/bin/qemu-system-x86_64 -enable-kvm \
      -drive if=pflash,format=raw,readonly=on,file=${ovmf}/FV/OVMF_CODE.fd \
      -drive if=pflash,format=raw,readonly=on,file=${ovmf}/FV/OVMF_VARS.fd \
      -drive format=raw,file=fat:rw:testesp
  '';

  devShell = with pkgs;
    mkShell {
      buildInputs = [ mtools rustToolchain pre-commit pkgsCross.stdenv.cc ];
      RUST_SRC_PATH = rustPlatform.rustLibSrc;
    };
}
