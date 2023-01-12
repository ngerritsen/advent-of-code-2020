use std::collections::HashMap;

fn main() {
    let nums: Vec<usize> = include_str!("input.txt")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", get_nth(&nums, 2020));
    println!("{}", get_nth(&nums, 30_000_000));
}

fn get_nth(nums: &[usize], n: usize) -> usize {
    let mut last_indices = nums
        .iter()
        .take(nums.len() - 1)
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect::<HashMap<usize, usize>>();

    let mut last_num = *nums.iter().last().unwrap();

    for i in nums.len()..n {
        let mut next_num = 0;
        let last_index = last_indices.get(&last_num);

        if let Some(last_idx) = last_index {
            next_num = i - last_idx - 1;
        }

        last_indices.insert(last_num, i - 1);
        last_num = next_num;
    }

    last_num
}
