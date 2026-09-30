pub fn nth(n: u32) -> u32 {
    let mut i = 3;
    let mut n = n;

    while n > 0 {
        if is_prime(i) {
            n -= 1;
        }
        i += 1
    }
    i -= 1;

    return i;
}

fn is_prime(k: u32) -> bool {
    if k <= 1 {
        return false;
    }

    if k == 2 || k == 3 {
        return true;
    }

    if k % 2 == 0 || k % 3 == 0 {
        return false;
    }

    let mut i = 5;

    while i * i <= k {
        if k % i == 0 || k % (i + 2) == 0 {
            return false;
        }

        i += 6
    }

    return true;
}
