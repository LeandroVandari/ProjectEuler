pub fn solve() -> String {
    std::env::set_current_dir(std::env::current_exe().unwrap().parent().unwrap()).unwrap();
    const FIRST_N: usize = 10;
    let answer = include_str!("p13_numbers")
        .split('\n')
        .map(|line| {
            line.chars()
                .take(FIRST_N + 2)
                .collect::<String>()
                .parse::<u128>()
                .unwrap()
        })
        .sum::<u128>()
        .to_string()
        .chars()
        .take(FIRST_N)
        .collect::<String>();

    answer
}
