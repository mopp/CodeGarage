#![no_std]
#![feature(lang_items)]

pub fn sample_func() {
}

#[cfg(not(test))]
#[lang = "eh_personality"]
pub extern fn eh_personality()
{
}


#[cfg(not(test))]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(_: core::fmt::Arguments, _: &'static str, _: u32) -> !
{
    loop {}
}


#[no_mangle]
pub extern fn abort()
{
    loop {}
}
