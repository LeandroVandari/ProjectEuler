fn main() {
    const RANGE_END: u32 = 20;
    let primes_to_range_end: Vec<u32> = (2..=RANGE_END).filter(|i| if *i != 2 {(2..*i).all(|j| i % j !=0)} else {true}).collect();
    
    let mut prime_factors_amount: Vec<(u32, u32)> = Vec::new();

    for prime in primes_to_range_end {
        let mut max_amount = 0;
        
        for number in 2..RANGE_END {
            let mut amount = 0;
            let mut curr_num = number;
            loop {
                if curr_num % prime != 0 {break;}

                curr_num /= prime;
                amount+=1;
            }
            if amount > max_amount {max_amount = amount;}
        }
        prime_factors_amount.push((prime, max_amount));
    }

    let smallest_dividable = prime_factors_amount.iter().map(|(prime, amount)| prime.pow(*amount)).fold(1u128, |acc, x| acc * x as u128);

    println!("{smallest_dividable}");
}
