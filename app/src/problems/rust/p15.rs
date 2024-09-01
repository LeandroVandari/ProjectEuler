pub fn solve() -> u128 {
    let mut values = std::collections::HashMap::new();
    take_step(&mut values, 0, 0)
}

fn take_step(
    values: &mut std::collections::HashMap<(u8, u8), u128>,
    curr_x: u8,
    curr_y: u8,
) -> u128 {
    if (curr_x, curr_y) == (20, 20) {
        return 1;
    }
    let mut num = 0;
    let branch_values = values.get(&(curr_x, curr_y));
    if let None = branch_values {
        let mut x = 0;
        let mut y = 0;
        if curr_x < 20 {
            let b = take_step(values, curr_x + 1, curr_y);
            x = b;
            num += b;
        }
        if curr_y < 20 {
            let b = take_step(values, curr_x, curr_y + 1);
            y = b;
            num += b;
        }
        values.insert((curr_x, curr_y), x + y);
    } else {
        num += branch_values.unwrap()
    }
    num
}
