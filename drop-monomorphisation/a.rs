#![no_std]

use core::fmt;

pub struct MyArc<T> {
    _value: T,
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        self.drop_slow()
    }
}

impl<T> MyArc<T> {
    #[inline(never)]
    fn drop_slow(&mut self) {
        unsafe {
            print(format_args!("dropping {:?}", core::any::type_name::<T>()));
        }
    }
}

#[allow(improper_ctypes)]
extern "C" {
    fn print(_: fmt::Arguments);
}
