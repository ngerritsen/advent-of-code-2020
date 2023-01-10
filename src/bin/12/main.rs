#[derive(Copy, Clone)]
struct Coord {
    x: isize,
    y: isize,
}

struct Ship {
    pos: Coord,
    vec: Coord,
}

struct Command {
    cmd: char,
    n: isize,
}

fn main() {
    let commands = parse_commands(include_str!("input.txt"));

    println!("{}", get_distance_v1(&commands));
    println!("{}", get_distance_v2(&commands));
}

fn get_distance_v1(commands: &[Command]) -> isize {
    commands
        .iter()
        .fold(Ship::from(Coord::from(1, 0)), |s, c| match c.cmd {
            'N' | 'E' | 'S' | 'W' => s.with_pos(s.pos.move_with(get_vec(&c.cmd), c.n)),
            _ => process_common_command(s, c),
        })
        .manhattan_distance()
}

fn get_distance_v2(commands: &[Command]) -> isize {
    commands
        .iter()
        .fold(Ship::from(Coord::from(10, -1)), |s, c| match c.cmd {
            'N' | 'E' | 'S' | 'W' => s.with_vec(s.vec.move_with(get_vec(&c.cmd), c.n)),
            _ => process_common_command(s, c),
        })
        .manhattan_distance()
}

fn process_common_command(ship: Ship, command: &Command) -> Ship {
    match command.cmd {
        'R' => ship.with_vec(ship.vec.rotate_cw(command.n)),
        'L' => ship.with_vec(ship.vec.rotate_ccw(command.n)),
        'F' => ship.with_pos(ship.pos.add(ship.vec.mul(command.n))),
        _ => panic!("Unknown command"),
    }
}

fn get_vec(cmd: &char) -> Coord {
    match cmd {
        'N' => Coord::from(0, -1),
        'E' => Coord::from(1, 0),
        'S' => Coord::from(0, 1),
        'W' => Coord::from(-1, 0),
        _ => panic!("Invalid coord"),
    }
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(cmd, n)| Command {
            cmd: cmd.chars().next().unwrap(),
            n: n.parse().unwrap(),
        })
        .collect()
}

impl Ship {
    fn with_pos(&self, pos: Coord) -> Ship {
        Ship { pos, vec: self.vec }
    }

    fn with_vec(&self, vec: Coord) -> Ship {
        Ship { pos: self.pos, vec }
    }

    fn manhattan_distance(&self) -> isize {
        self.pos.x.abs() + self.pos.y.abs()
    }

    fn from(vec: Coord) -> Ship {
        Ship {
            pos: Coord::from(0, 0),
            vec,
        }
    }
}

impl Coord {
    fn from(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    fn add(&self, d: Coord) -> Coord {
        Coord::from(self.x + d.x, self.y + d.y)
    }

    fn move_with(&self, d: Coord, n: isize) -> Coord {
        self.add(d.mul(n))
    }

    fn mul(&self, m: isize) -> Coord {
        Coord::from(self.x * m, self.y * m)
    }

    fn rotate_cw(&self, deg: isize) -> Coord {
        (0..deg / 90).fold(*self, |c, _| Coord::from(-c.y, c.x))
    }

    fn rotate_ccw(&self, deg: isize) -> Coord {
        (0..deg / 90).fold(*self, |c, _| Coord::from(c.y, -c.x))
    }
}
