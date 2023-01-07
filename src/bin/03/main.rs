use std::str::Lines;

fn main() {
    let lines = include_str!("input.txt").lines();

    println!("{}", get_tree_count(&lines, 3, 1));
    println!("{}", get_total_tree_count(&lines));
}

fn get_total_tree_count(lines: &Lines) -> usize {
    Vec::from([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
        .iter()
        .map(|(x, y)| get_tree_count(lines, *x as usize, *y as usize))
        .product()
}

fn get_tree_count(lines: &Lines, x: usize, y: usize) -> usize {
    lines
        .clone()
        .enumerate()
        .step_by(y)
        .filter(|(i, l)| l.chars().nth((i / y * x) % l.len()) == Some('#'))
        .count()
}
