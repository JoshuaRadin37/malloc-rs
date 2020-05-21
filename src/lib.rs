#![no_std]

extern crate alloc;

pub mod mem_sizes;
pub mod allocator;

pub mod paging;

extern crate libc;
extern {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[cfg(debug_assertions)]
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use alloc::format;
    let string = format!("{}", args).as_str().as_ptr();
    unsafe { printf(string) };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! print {
    ($($expr:tt)+) => {
        $crate::_print(format_args!($($expr),+))
    };
}
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($expr:tt)*) => {
        $crate::_print(format_args!($($expr)*))
    };
}

#[cfg(not(debug_assertions))]
macro_rules! print {
    () => {

    };
}
#[cfg(not(debug_assertions))]
macro_rules! println {
   () => {
    };
    ($($expr:tt)*) => {
    };
}