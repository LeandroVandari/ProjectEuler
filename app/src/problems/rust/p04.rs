pub fn solve() -> u32 {
    let mut largest_palindrome: u32 = 0;
    for i in 1..=999 {
        for j in i..=999 {
            let num = i * j;
            let num_str = num.to_string();
            if let std::cmp::Ordering::Equal = num_str.chars().cmp(num_str.chars().rev()) {
                if num > largest_palindrome {
                    largest_palindrome = num;
                }
            }
        }
    }
    largest_palindrome
}
