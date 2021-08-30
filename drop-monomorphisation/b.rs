#![no_std]

pub struct S {
    _inner: a::MyArc<i32>,
}

// use core::mem::ManuallyDrop;

// pub struct S {
//     inner: ManuallyDrop<a::MyArc<i32>>,
// }

// impl Drop for S {
//     fn drop(&mut self) {
//         unsafe {
//             ManuallyDrop::drop(&mut self.inner)
//         }
//     }
// }
