pub fn solve() ->   String {
    let mut num: u128 = 600851475143;
    let mut primes = vec![2];
    for i in (3..=600851475143).step_by(2) {
        let mut is_prime = true;
        for prime in &primes {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
            if num % i == 0 {
                num /= i;
            }
        }
        if num == 1 {
            return i.to_string();
        }
    }

    unreachable!()
}
