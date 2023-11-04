fn main() {
    const FIRST_N: usize = 10;
    let answer = std::fs::read_to_string("13_numbers").expect("No file idiot").split('\n').map(|line| line.chars().take(FIRST_N + 2).collect::<String>().parse::<u128>().unwrap()).sum::<u128>().to_string().chars().take(FIRST_N).collect::<String>();

    println!("{answer}");
}