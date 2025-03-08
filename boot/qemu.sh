#!/usr/bin/env bash

set -euxo pipefail

# set up an esp
cargo build --target x86_64-unknown-uefi --release
rm -rf esp
mkdir -p esp/efi/boot
cp -r target/x86_64-unknown-uefi/release/astridos-bootos.efi esp/efi/boot/bootx64.efi

# copy ovmfies in
ovmfdir=$(nix build .#debug.boot.ovmf.fd --print-out-paths --no-link)
rm -f *.fd
cp $ovmfdir/FV/* .
chmod +rw *.fd

# launch!
sudo qemu-system-x86_64 \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp \
    -net nic,model=virtio,macaddr=52:54:00:00:00:01 -net bridge,br=virbr0