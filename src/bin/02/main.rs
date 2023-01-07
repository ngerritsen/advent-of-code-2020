use itertools::Itertools;

struct Password<'a> {
    password: &'a str,
    min: usize,
    max: usize,
    char: char,
}

fn main() {
    let passwords: Vec<_> = include_str!("input.txt")
        .lines()
        .into_iter()
        .map(parse_password)
        .collect();

    let valid_v1 = passwords.iter().filter(|p| p.is_valid_v1()).count();
    let valid_v2 = passwords.iter().filter(|p| p.is_valid_v2()).count();

    println!("{}", valid_v1);
    println!("{}", valid_v2);
}

fn parse_password(str: &str) -> Password {
    let (range, char, password) = str.split(' ').collect_tuple().unwrap();
    let (min, max) = range
        .split('-')
        .map(|v| v.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    Password {
        password,
        min,
        max,
        char: char.chars().next().unwrap(),
    }
}

impl Password<'_> {
    fn is_valid_v1(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.char).count();
        self.min <= count && count <= self.max
    }

    fn is_valid_v2(&self) -> bool {
        (self.password.chars().nth(self.min - 1).unwrap_or('-') == self.char)
            ^ (self.password.chars().nth(self.max - 1).unwrap_or('-') == self.char)
    }
}
