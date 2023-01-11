fn main() {
    let (time, buses) = parse_input(include_str!("input.txt"));

    println!("{}", get_earliest_bus(&buses, time));
    println!("{}", get_earliest_subsequent_timestamp(&buses))
}

fn get_earliest_bus(buses: &[usize], time: usize) -> usize {
    let (id, d) = buses
        .iter()
        .filter(|t| **t != 0)
        .map(|t| (t, t - (time % t)))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    id * d
}

fn get_earliest_subsequent_timestamp(buses: &[usize]) -> usize {
    let mut lcm = buses[0];
    let mut ans = 0;

    for (i, &bus) in buses.iter().enumerate() {
        if bus == 0 || i == 0 {
            continue;
        }

        ans = get_match_pos(ans, lcm, bus, i);
        lcm = get_lcm(lcm, bus)
    }

    ans
}

fn get_match_pos(mut pos: usize, a: usize, b: usize, offset: usize) -> usize {
    while pos % b != b - (offset % b) {
        pos += a
    }

    pos
}

fn get_lcm(a: usize, b: usize) -> usize {
    (a / get_gdc(a, b)) * b
}

fn get_gdc(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        get_gdc(b, a % b)
    }
}

fn parse_input(input: &str) -> (usize, Vec<usize>) {
    (
        input.lines().next().unwrap().parse().unwrap(),
        input
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|x| if x == "x" { 0 } else { x.parse().unwrap() })
            .collect(),
    )
}
