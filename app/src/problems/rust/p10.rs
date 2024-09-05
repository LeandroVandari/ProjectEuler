pub fn solve() -> String {
    const UNTIL: u128 = 2000000;
    let mut primes = vec![2, 3];
    let mut sum = 5;

    for i in (5..=UNTIL).step_by(2) {
        let mut is_prime = true;
        for prime in &primes {
            if prime > &((i as f64).powf(0.5) as u128) {
                break;
            }
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
            sum += i;
        }
    }

    sum.to_string()
}
