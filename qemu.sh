#!/bin/bash
set -euo pipefail

mkdir -p target/vfat
cp "$1" target/vfat
cp startup.nsh target/vfat

if [ ! -e OVMF_VARS_4M.fd ]; then
    cp /usr/share/OVMF/OVMF_VARS_4M.fd .
fi

qemu-system-x86_64 \
    -enable-kvm -cpu host -m 4G -s \
    -drive if=pflash,unit=0,format=raw,read-only=on,file=/usr/share/OVMF/OVMF_CODE_4M.fd \
    -drive if=pflash,unit=1,format=raw,file=OVMF_VARS_4M.fd \
    -drive format=raw,file=fat:rw:target/vfat \
    -serial stdio -parallel null -display none || true
