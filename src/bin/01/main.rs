use std::collections::HashSet;

const TARGET: i32 = 2020;

fn main() {
    let input = include_str!("input.txt");
    let nums = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<HashSet<i32>>();

    println!("{}", get_pair(&nums));
    println!("{}", get_trio(&nums));
}

fn get_pair(nums: &HashSet<i32>) -> i32 {
    let left = nums.iter().find(|v| nums.contains(&(TARGET - *v))).unwrap();

    left * (TARGET - left)
}

fn get_trio(nums: &HashSet<i32>) -> i32 {
    let (left, right) = nums
        .iter()
        .map(|v| {
            (
                v,
                nums.iter()
                    .find(|w| nums.contains(&(TARGET - *v - *w)))
                    .unwrap_or(&0),
            )
        })
        .find(|(_, b)| **b != 0)
        .unwrap();

    left * right * (TARGET - left - right)
}
