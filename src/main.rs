#![no_std]
#![no_main]
#![feature(lang_items)]

// use core::assert;

static RANDOM_UNACCESSED: u32 = 12;
static RANDOM_ACCESSED: u32 = 55;
static RANDOM_STRING: &str = "random";

use core::arch::asm;

#[no_mangle]
pub fn _start() -> ! {
    // assert!(RANDOM_UNACCESSED > 0);
    exit(RANDOM_ACCESSED as u64);
    // exit(RANDOM_STRING.chars().next().unwrap() as u64);
}

/// Exit syscall
pub fn exit(_code: u64) -> ! {
    unsafe {
        // a0 is _code
        asm!("li a7, 93");
        asm!("ecall");
    }
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// TODO: we can remove this once https://github.com/rust-lang/rust/issues/85736 is fixed.
#[no_mangle]
pub fn __atomic_load_4(arg: *const usize, _ordering: usize) -> usize {
    unsafe { *arg }
}


// 000110b4 <_start>:
//    110b4:       03700513                li      a0,55
//    110b8:       00000593                li      a1,0
//    110bc:       00000097                auipc   ra,0x0
//    110c0:       00c080e7                jalr    12(ra) # 110c8 <_ZN13mem_init_prog4exit17h551f38a2d681ef3bE>
//    110c4:       c0001073                unimp

// 000110c8 <_ZN13mem_init_prog4exit17h551f38a2d681ef3bE>:
//    110c8:       ff010113                add     sp,sp,-16
//    110cc:       00b12623                sw      a1,12(sp)
//    110d0:       00a12423                sw      a0,8(sp)
//    110d4:       05d00893                li      a7,93
//    110d8:       00000073                ecall
//    110dc:       0040006f                j       110e0 <_ZN13mem_init_prog4exit17h551f38a2d681ef3bE+0x18>