use std::collections::HashMap;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = Vec::new();

    let digit_map: HashMap<u32, &str> = vec![
        (0u32, "No"),
        (1u32, "One"),
        (2u32, "Two"),
        (3u32, "Three"),
        (4u32, "Four"),
        (5u32, "Five"),
        (6u32, "Six"),
        (7u32, "Seven"),
        (8u32, "Eight"),
        (9u32, "Nine"),
        (10u32, "Ten"),
    ]
    .iter()
    .map(|&t| t)
    .collect();

    let mut remained_bottles = start_bottles;

    for i in 0..take_down {
        let bottle_hanging_phrase = if remained_bottles == 1 {
            format!("{} green bottle", digit_map[&remained_bottles])
        } else {
            format!("{} green bottles", digit_map[&remained_bottles])
        };

        let remained_bottle_phrase = if remained_bottles - 1 == 1 {
            format!(
                "{} green bottle",
                digit_map[&(remained_bottles - 1)].to_lowercase()
            )
        } else {
            format!(
                "{} green bottles",
                digit_map[&(remained_bottles - 1)].to_lowercase()
            )
        };

        let new_line = if take_down > 1 && i != take_down - 1 {
            "\n\n"
        } else {
            ""
        };

        let lyric = format!(
            "{} hanging on the wall,
{} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {} hanging on the wall.{}",
            bottle_hanging_phrase, bottle_hanging_phrase, remained_bottle_phrase, new_line
        );

        song.push(lyric);
        remained_bottles -= 1;
    }

    song.into_iter().collect()
}
