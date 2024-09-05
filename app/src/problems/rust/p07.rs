pub fn solve() -> String {
    let mut primes: Vec<u32> = vec![2, 3];
    let mut num: u32 = 2;
    let mut last_prime: u32 = 3;
    while num != 10001 {
        let mut i: u32 = last_prime;
        loop {
            let mut is_prime = true;
            i += 2;
            for prime in &primes {
                if (*prime as f32) > (i as f32).powf(0.5) {
                    break;
                }
                if i % prime == 0 {
                    is_prime = false;
                }
            }
            if is_prime {
                primes.push(i);
                last_prime = i;
                num += 1;
                break;
            }
        }
    }
    primes[10000].to_string()
}
