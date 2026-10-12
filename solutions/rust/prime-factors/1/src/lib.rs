pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();

    let mut divisor = 2;
    let mut num = n;

    while divisor <= num || num != 1 {
        if num % divisor == 0 {
            prime_factors.push(divisor);
            let quotient = num / divisor;
            if quotient == 1 {
                break;
            } else {
                num = quotient;
            }
        } else {
            if divisor == 2 {
                divisor = 3;
            } else {
                divisor += 2;
            }
        }
    }

    prime_factors
}
