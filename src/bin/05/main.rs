use std::collections::HashSet;

fn main() {
    let seat_ids = include_str!("input.txt")
        .lines()
        .map(get_seat_id)
        .collect::<HashSet<u32>>();

    let max_seat_id = seat_ids.iter().max().unwrap();
    let my_seat_id = seat_ids
        .iter()
        .find(|id| !seat_ids.contains(&(*id + 1)) && seat_ids.contains(&(*id + 2)))
        .unwrap()
        + 1;

    println!("{}", max_seat_id);
    println!("{}", my_seat_id);
}

fn get_seat_id(code: &str) -> u32 {
    let (row, col) = code.split_at(7);
    (parse_bsp(row, 'F') * 8) + parse_bsp(col, 'L')
}

fn parse_bsp(code: &str, low: char) -> u32 {
    let start = (0, 2_i32.pow(code.len() as u32) - 1);
    let (ans, _) = code.chars().into_iter().fold(start, |(min, max), c| {
        let step = (max - min + 1) / 2;
        if c == low {
            (min, max - step)
        } else {
            (min + step, max)
        }
    });
    ans as u32
}
