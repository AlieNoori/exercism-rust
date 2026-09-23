pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    if height == 0 {
        return vec![];
    }
    let width = garden[0].len();

    let mut counts = vec![vec![0u32; width]; height];
    let mut is_flower = vec![vec![false; width]; height];

    let dir = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (y, &row) in garden.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == '*' {
                is_flower[y][x] = true;

                for (dy, dx) in &dir {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;

                    if ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize {
                        counts[ny as usize][nx as usize] += 1;
                    }
                }
            }
        }
    }

    let mut result = Vec::with_capacity(height);
    for y in 0..height {
        let mut row_str = String::with_capacity(width);
        for x in 0..width {
            if is_flower[y][x] {
                row_str.push('*');
            } else if counts[y][x] > 0 {
                let c = char::from_digit(counts[y][x], 10).unwrap();
                row_str.push(c);
            } else {
                row_str.push(' ');
            }
        }
        result.push(row_str);
    }

    result
}
