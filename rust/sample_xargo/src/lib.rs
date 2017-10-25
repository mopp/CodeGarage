#![no_std]
#![feature(lang_items)]

#[cfg(test)]
#[macro_use]
extern crate std;

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
    }
}
