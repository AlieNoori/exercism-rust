/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() <= 1 || !code.chars().all(|c| c.is_digit(10) || c == ' ') {
        return false;
    }

    let stripped_code: String = code.chars().filter(|c| c.is_digit(10)).collect();
    if stripped_code.len() <= 1 {
        return false;
    }

    let mut digits: Vec<u32> = Vec::with_capacity(stripped_code.len());

    for (i, c) in stripped_code.chars().rev().enumerate() {
        if i % 2 == 0 {
            digits.push(c.to_digit(10).unwrap());
        } else {
            let digit = c.to_digit(10).unwrap();
            if digit * 2 > 9 {
                digits.push(digit * 2 - 9);
            } else {
                digits.push(digit * 2);
            }
        }
    }

    digits.iter().sum::<u32>() % 10 == 0
}
