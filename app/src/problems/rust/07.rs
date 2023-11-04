fn main() {
    let mut primes = vec![2, 3];
    let mut num = 2;
    let mut last_prime = 3;
    while num != 10001 {
        let mut i: i32 = last_prime;
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
                num +=1;
                break;
            }
        }
    }
    println!("{}", primes[10000]);
}
