pub fn solve() -> u32 {
    let mut cur = 1;
    let mut prev = 1;
    let mut sum = 0;

    while cur < 4_000_000 {
        let a = cur;
        cur += prev;
        prev = a;
        if cur % 2 == 0 {
            sum += cur;
        }
    }
    sum
}
