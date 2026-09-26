pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digit(num);
    let power = digits.len();

    let sum: u32 = digits.iter().map(|d| d.pow(power as u32)).sum();

    sum == num
}

fn get_digit(num: u32) -> Vec<u32> {
    if num == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    let mut num = num;

    while num != 0 {
        let remainder = num % 10;
        digits.push(remainder);

        num = num / 10;
    }

    digits
}
