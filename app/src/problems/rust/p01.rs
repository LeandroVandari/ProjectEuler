pub fn solve() -> String {
    (1..1000).filter(|num| num % 3 == 0 || num % 5 == 0).sum::<u32>().to_string()
}