target remote :1234
source ./load-symbols.py
load-symbols $rip "./target/uefi-runtime-driver-debug/debug/rust-uefi-runtime-driver-debug.efi"
set GDB_ATTACHED = 1
