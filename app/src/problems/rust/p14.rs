pub fn solve() -> u128 {
    const UP_UNTIL: u128 = 1000000;

    let mut max_amount = 0;
    let mut max_num = 0;
    let mut starting_num = UP_UNTIL / 2;
    if starting_num % 2 == 0 {
        starting_num += 1;
    }
    for i in (starting_num..=UP_UNTIL).step_by(2) {
        let collatz_amount = collatz_sequence(i);
        if collatz_amount > max_amount {
            max_amount = collatz_amount;
            max_num = i;
        }
    }
    max_num
}

fn collatz_sequence(num: u128) -> u32 {
    if num == 1 {
        return 1;
    }
    if num % 2 == 0 {
        return 1 + collatz_sequence(num / 2);
    } else {
        return 2 + collatz_sequence((num * 3 + 1) / 2);
    }
}
