#!/usr/bin/env bash

set -euxo pipefail

cargo build --target x86_64-unknown-uefi --release
rm -r esp
mkdir -p esp/efi/boot
cp -r target/x86_64-unknown-uefi/release/astridos-bootos.efi esp/efi/boot/bootx64.efi
qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp