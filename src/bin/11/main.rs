use itertools::{merge, Itertools};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd)]
struct Coord {
    x: isize,
    y: isize,
}

type Seats = HashSet<Coord>;
type Views = HashMap<Coord, Vec<Coord>>;

struct Area {
    empty: Seats,
    occupied: Seats,
    width: isize,
    height: isize,
}

fn main() {
    let input = include_str!("input.txt");

    println!("{}", find_stable_occupation(parse_area(input), false));
    println!("{}", find_stable_occupation(parse_area(input), true));
}

fn find_stable_occupation(mut area: Area, full: bool) -> usize {
    let views = generate_views(&area, full);
    let mut occupation = area.occupied.len();

    loop {
        let next_area = play_round(area, &views, full);

        if next_area.occupied.len() == occupation {
            return occupation;
        }

        occupation = next_area.occupied.len();
        area = next_area;
    }
}

fn play_round(area: Area, views: &Views, full: bool) -> Area {
    let mut next_empty = Seats::new();
    let mut next_occupied = Seats::new();

    area.empty.iter().for_each(|c| {
        if should_occupy(&area, c, views) {
            next_occupied.insert(*c);
        } else {
            next_empty.insert(*c);
        }
    });

    area.occupied.iter().for_each(|c| {
        if should_empty(&area, c, views, full) {
            next_empty.insert(*c);
        } else {
            next_occupied.insert(*c);
        }
    });

    area.with_seats(next_empty, next_occupied)
}

fn should_occupy(area: &Area, seat: &Coord, views: &Views) -> bool {
    views
        .get(seat)
        .unwrap()
        .iter()
        .all(|c| !area.occupied.contains(c))
}

fn should_empty(area: &Area, seat: &Coord, views: &Views, full: bool) -> bool {
    let limit = if full { 5 } else { 4 };
    views
        .get(seat)
        .unwrap()
        .iter()
        .filter(|c| area.occupied.contains(c))
        .take(limit)
        .count()
        >= limit
}

fn generate_views(area: &Area, full: bool) -> Views {
    area.empty
        .iter()
        .map(|c| (*c, get_visible_seats(area, c, full)))
        .collect::<Views>()
}

fn get_visible_seats(area: &Area, seat: &Coord, full: bool) -> Vec<Coord> {
    get_visible_directions()
        .iter()
        .map(|d| {
            if full {
                get_first_visible_in_direction(area, seat, d)
            } else {
                seat.add(*d)
            }
        })
        .filter(|c| area.empty.contains(c))
        .collect_vec()
}

fn get_first_visible_in_direction(area: &Area, seat: &Coord, direction: &Coord) -> Coord {
    let mut step = 1;

    loop {
        let coord = seat.add(direction.mul(step));

        if area.empty.contains(&coord) || !area.contains(&coord) {
            return coord;
        }

        step += 1;
    }
}

fn get_visible_directions() -> Vec<Coord> {
    merge(
        (-1..2).permutations(2).map(|v| Coord::from(v[0], v[1])),
        [Coord::from(-1, -1), Coord::from(1, 1)],
    )
    .collect_vec()
}

fn parse_area(input: &str) -> Area {
    Area {
        empty: input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == 'L')
                    .map(move |(x, _)| Coord {
                        x: x as isize,
                        y: y as isize,
                    })
            })
            .collect::<Seats>(),
        occupied: Seats::new(),
        height: input.lines().count() as isize,
        width: input.lines().next().unwrap().len() as isize,
    }
}

fn between(v: isize, a: isize, b: isize) -> bool {
    v >= a && v <= b
}

impl Area {
    fn with_seats(&self, empty: Seats, occupied: Seats) -> Area {
        Area {
            empty,
            occupied,
            width: self.width,
            height: self.height,
        }
    }

    fn contains(&self, coord: &Coord) -> bool {
        between(coord.x, 0, self.width - 1) && between(coord.y, 0, self.height)
    }
}

impl Coord {
    fn from(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    fn add(&self, d: Coord) -> Coord {
        Coord::from(self.x + d.x, self.y + d.y)
    }

    fn mul(&self, m: isize) -> Coord {
        Coord::from(self.x * m, self.y * m)
    }
}
