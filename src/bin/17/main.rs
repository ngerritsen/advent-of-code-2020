use std::collections::HashSet;

use itertools::Itertools;
use memoize::memoize;

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Hypercube {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

fn main() {
    let source_cubes = parse_cubes(include_str!("input.txt"));
    let cubes = (0..6).fold(source_cubes.clone(), |prev, _| run_cycle_3d(prev));
    let hyper_cubes = (0..6).fold(to_hyper_cubes(&source_cubes), |prev, _| run_cycle_4d(prev));

    println!("{}", cubes.len());
    println!("{}", hyper_cubes.len());
}

fn run_cycle_3d(cubes: HashSet<Cube>) -> HashSet<Cube> {
    let get_active_neighbours = |c: &Cube| {
        c.neighbours()
            .iter()
            .filter(|c| cubes.contains(*c))
            .take(4)
            .count()
    };

    let turned_on = cubes
        .iter()
        .flat_map(|c| c.neighbours())
        .filter(|c| 3 == get_active_neighbours(c))
        .collect_vec();

    let stay_on = cubes
        .iter()
        .filter(|c| (2..4).contains(&get_active_neighbours(c)))
        .copied()
        .collect_vec();

    [stay_on, turned_on].concat().iter().copied().collect()
}

fn run_cycle_4d(cubes: HashSet<Hypercube>) -> HashSet<Hypercube> {
    let get_active_neighbours = |c: &Hypercube| {
        c.neighbours()
            .iter()
            .filter(|c| cubes.contains(*c))
            .take(4)
            .count()
    };

    let turned_on = cubes
        .iter()
        .flat_map(|c| c.neighbours())
        .filter(|c| 3 == get_active_neighbours(c))
        .collect_vec();

    let stay_on = cubes
        .iter()
        .filter(|c| (2..4).contains(&get_active_neighbours(c)))
        .copied()
        .collect_vec();

    [stay_on, turned_on].concat().iter().copied().collect()
}

#[memoize]
fn get_neighbours_seed(n: usize) -> Vec<Vec<isize>> {
    (-1..2)
        .combinations_with_replacement(n)
        .collect_vec()
        .iter()
        .flat_map(|c| c.iter().permutations(n))
        .unique()
        .map(|c| c.iter().map(|v| **v).collect_vec())
        .filter(|c| !c.iter().all(|v| *v == 0))
        .collect()
}

#[memoize]
fn get_neighbours_3d(cube: Cube) -> Vec<Cube> {
    get_neighbours_seed(3)
        .iter()
        .map(|v| {
            Cube {
                x: v[0],
                y: v[1],
                z: v[2],
            }
            .add(&cube)
        })
        .collect()
}

#[memoize]
fn get_neighbours_4d(cube: Hypercube) -> Vec<Hypercube> {
    get_neighbours_seed(4)
        .iter()
        .map(|v| {
            Hypercube {
                x: v[0],
                y: v[1],
                z: v[2],
                w: v[3],
            }
            .add(&cube)
        })
        .collect()
}

fn to_hyper_cubes(cubes: &HashSet<Cube>) -> HashSet<Hypercube> {
    cubes.iter().map(Hypercube::from).collect()
}

fn parse_cubes(input: &str) -> HashSet<Cube> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Cube {
                    x: x as isize,
                    y: y as isize,
                    z: 0,
                })
        })
        .collect()
}

impl Cube {
    fn add(&self, b: &Self) -> Self {
        Self {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }

    fn neighbours(&self) -> Vec<Self> {
        get_neighbours_3d(*self)
    }
}

impl Hypercube {
    fn from(cube: &Cube) -> Self {
        Self {
            x: cube.x,
            y: cube.y,
            z: cube.z,
            w: 0,
        }
    }

    fn add(&self, b: &Self) -> Self {
        Self {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
            w: self.w + b.w,
        }
    }

    fn neighbours(&self) -> Vec<Self> {
        get_neighbours_4d(*self)
    }
}
