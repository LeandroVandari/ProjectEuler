fn main() {
    let mut current_sum = 1;
    let mut current_num = 1;
    while divisors_amount(current_sum) <= 500 {
        current_num += 1;
        current_sum += current_num;
    }
    println!("{current_sum}");
}

fn divisors_amount(num: u128) -> u128 {
    let mut div_amount = 0;
    for i in 1..=((num as f64).powf(0.5) as u128) {
        if i * i == num {
            div_amount += 1;
        }
        else if num % i == 0 {
            div_amount += 2;
        }
    }
    div_amount

}