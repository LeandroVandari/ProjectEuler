pub fn solve() -> String {
    const SUM: u32 = 1000;
    for a in 1..=499 {
        for b in a..=499 {
            let c = SUM - (a + b);
            if c * c == a * a + b * b {
                return (a * b * c).to_string();
            }
        }
    }
    unreachable!()
}
