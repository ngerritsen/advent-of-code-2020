use itertools::Itertools;

fn main() {
    let groups = include_str!("input.txt").split("\n\n").collect_vec();

    let total_yes = groups
        .iter()
        .map(|s| s.chars().filter(|c| c.is_alphabetic()).unique().count())
        .sum::<usize>();

    let total_all_yes = groups
        .iter()
        .map(|s| {
            s.chars()
                .counts()
                .iter()
                .filter(|(_, n)| **n == s.lines().count())
                .count()
        })
        .sum::<usize>();

    println!("{}", total_yes);
    println!("{}", total_all_yes);
}
