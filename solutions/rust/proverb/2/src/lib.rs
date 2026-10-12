use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    let Some(first) = list.first() else {
        return String::new();
    };

    let mut proverb = String::new();

    for pair in list.windows(2) {
        writeln!(
            proverb,
            "For want of a {} the {} was lost.",
            pair[0], pair[1]
        )
        .unwrap();
    }

    write!(proverb, "And all for the want of a {}.", first).unwrap();

    proverb
}
