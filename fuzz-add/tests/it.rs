#[test]
fn fuzz_add() {
    bolero::check!()
        .with_type()
        .cloned()
        .for_each(|(a, b)| fuzz_add::add(a, b) == a.wrapping_add(b));
}
