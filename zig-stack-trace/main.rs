#[inline(never)]
fn foo(x: u32) -> u32 {
    x * x
}

#[inline(never)]
fn bar() -> u32 {
    return foo(u32::MAX);
}

pub fn main() {
    println!("{}", bar());
}

