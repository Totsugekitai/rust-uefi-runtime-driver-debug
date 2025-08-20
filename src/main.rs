#![no_main]
#![no_std]

mod allocator;

extern crate alloc;

use alloc::vec::Vec;
use r_efi::efi;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

unsafe fn write_to_port(port: u16, value: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

static mut GDB_ATTACHED: bool = false;

#[unsafe(export_name = "efi_main")]
pub extern "C" fn main(_h: efi::Handle, _st: *mut efi::SystemTable) -> efi::Status {
    #[cfg(debug_assertions)]
    unsafe {
        while !GDB_ATTACHED {
            core::arch::asm!("pause");
        }
    }

    let mut v = Vec::new();
    for c in b"hello world!\r\n" {
        v.push(*c);
    }

    for c in v {
        unsafe {
            write_to_port(0x3f8, c);
        }
    }

    efi::Status::SUCCESS
}
