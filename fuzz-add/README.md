# Fuzzing edge case

```rust
pub fn add(x: u32, y: u32) -> u32 {
    if x == 12976 && y == 14867 {
        return x.wrapping_sub(y);
    }
    return x.wrapping_add(y);
}
```

Fuzzer is able to find the offending x and y.

```rust
#[test]
fn fuzz_add() {
    bolero::check!()
        .with_type()
        .cloned()
        .for_each(|(a, b)| fuzz_add::add(a, b) == a.wrapping_add(b));
}
```

```console
$ cargo install cargo-bolero
$ cargo bolero test fuzz_add

# One minute later ...

======================== Test Failure ========================

Input:
(
    12976,
    14867,
)

Error:
test returned `false`
```
