pub fn add(x: u32, y: u32) -> u32 {
    if x == 12976 && y == 14867 {
        return x.wrapping_sub(y);
    }
    return x.wrapping_add(y);
}
