use std::{collections::HashMap, vec};

const SEA_MONSTER: &str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

#[derive(Clone, PartialEq, Eq)]
struct Tile {
    id: u64,
    edges: Vec<String>,
    grid: Grid,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

struct Mask {
    coords: Vec<Coord>,
    size: Size,
}

struct Size {
    width: usize,
    height: usize,
}

type TilePositions = HashMap<Coord, Tile>;
type TileGrid = Vec<Vec<Tile>>;
type Grid = Vec<Vec<char>>;

fn main() {
    let tiles = parse_tiles(include_str!("input.txt"));
    let tile_grid = arrange_tiles(&tiles);

    println!("{}", mul_corner_ids(&tile_grid));
    println!("{}", count_waves(&tile_grid));
}

fn count_waves(tile_grid: &TileGrid) -> usize {
    overlay_sea_monsters(&render_grid(tile_grid))
        .iter()
        .flatten()
        .filter(|c| **c == '#')
        .count()
}

fn mul_corner_ids(tiles: &TileGrid) -> u64 {
    let (width, height) = (tiles.iter().next().unwrap().len(), tiles.len());

    tiles[0][0].id
        * tiles[0][width - 1].id
        * tiles[height - 1][0].id
        * tiles[height - 1][width - 1].id
}

fn arrange_tiles(tiles: &[Tile]) -> TileGrid {
    let mut unarranged_tiles = tiles.to_vec();
    let mut arranged_tiles: TilePositions = HashMap::new();

    arranged_tiles.insert(Coord::from(0, 0), unarranged_tiles.pop().unwrap());

    while !unarranged_tiles.is_empty() {
        (arranged_tiles, unarranged_tiles) =
            position_next_grid_tile(&arranged_tiles, &unarranged_tiles);
    }

    to_tile_grid(&arranged_tiles)
}

fn position_next_grid_tile(
    arranged_tiles: &TilePositions,
    unarranged_tiles: &[Tile],
) -> (TilePositions, Vec<Tile>) {
    for (pos, tile) in arranged_tiles.iter() {
        for (direction, edge) in tile.edges.iter().enumerate() {
            let next_pos = pos.add(get_vec(direction));

            if arranged_tiles.contains_key(&next_pos) {
                continue;
            }

            for t in unarranged_tiles.iter() {
                for tt in vec![t, &t.flip()].iter() {
                    for (d, e) in tt.edges.iter().enumerate() {
                        if &reverse(e) == edge {
                            let rot = get_rot(direction, (d + 2) % 4);
                            return (
                                with_tile(arranged_tiles, next_pos, tt.rotate(rot)),
                                without_tile(unarranged_tiles, t.id),
                            );
                        }
                    }
                }
            }
        }
    }

    panic!("No matching tiles found");
}

fn overlay_sea_monsters(grid: &Grid) -> Grid {
    let mut next_grid = grid.clone();

    for _ in 0..2 {
        for _ in 0..4 {
            if let Some(result) = try_overlay_sea_monsters(&next_grid) {
                return result;
            }
            next_grid = grid.rotate();
        }
        next_grid = grid.flip()
    }

    panic!("Cannot find any sea monsters!")
}

fn try_overlay_sea_monsters(grid: &Grid) -> Option<Grid> {
    let mut next_grid = grid.clone();
    let size = grid.get_size();
    let sea_monster = parse_sea_monster();
    let mut found_any = false;

    for y in 0..(size.height - sea_monster.size.height) {
        for x in 0..(size.width - sea_monster.size.width) {
            let found = sea_monster
                .coords
                .iter()
                .all(|c| grid[y + c.y as usize][x + c.x as usize] == '#');

            if found {
                found_any = true;

                for c in sea_monster.coords.iter() {
                    next_grid[y + c.y as usize][x + c.x as usize] = 'O';
                }
            }
        }
    }

    if found_any {
        Some(next_grid)
    } else {
        None
    }
}

fn parse_sea_monster() -> Mask {
    let grid: Grid = SEA_MONSTER.lines().map(|l| l.chars().collect()).collect();
    let size = grid.get_size();
    let coords = grid
        .iter()
        .enumerate()
        .flat_map(move |(y, r)| {
            r.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| Coord::from(x as isize, y as isize))
        })
        .collect();

    Mask { coords, size }
}

fn with_tile(tiles: &TilePositions, coord: Coord, tile: Tile) -> HashMap<Coord, Tile> {
    let mut next_tiles = tiles.clone();
    next_tiles.insert(coord, tile);
    next_tiles
}

fn without_tile(tiles: &[Tile], id: u64) -> Vec<Tile> {
    tiles.iter().cloned().filter(|t| t.id != id).collect()
}

fn to_tile_grid(tiles: &TilePositions) -> TileGrid {
    let min_x = tiles.keys().map(|c| c.x).min().unwrap();
    let max_x = tiles.keys().map(|c| c.x).max().unwrap();
    let min_y = tiles.keys().map(|c| c.y).min().unwrap();
    let max_y = tiles.keys().map(|c| c.y).max().unwrap();

    (min_y..(max_y + 1))
        .map(|y| {
            (min_x..(max_x + 1))
                .map(|x| tiles.get(&Coord { x, y }).unwrap().clone())
                .collect()
        })
        .collect()
}

fn render_grid(grid: &TileGrid) -> Grid {
    grid.iter()
        .flat_map(|row| {
            let stripped_sub_grids = row.iter().map(|t| t.grid.strip()).collect::<Vec<Grid>>();

            let row = stripped_sub_grids
                .first()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(y, _)| {
                    stripped_sub_grids
                        .iter()
                        .flat_map(|sg| sg.get(y).unwrap())
                        .cloned()
                        .collect()
                })
                .collect::<Vec<Vec<char>>>();
            row
        })
        .collect()
}

fn get_rot(origin: usize, other: usize) -> usize {
    let diff: isize = origin as isize - other as isize;

    if diff < 0 {
        (4 + diff) as usize
    } else {
        diff as usize
    }
}

fn get_vec(direction: usize) -> Coord {
    match direction {
        1 => Coord::from(1, 0),
        0 => Coord::from(0, -1),
        2 => Coord::from(0, 1),
        3 => Coord::from(-1, 0),
        _ => panic!("Invalid direction"),
    }
}

fn reverse(s: &str) -> String {
    s.to_owned().chars().rev().collect::<String>()
}

impl Tile {
    fn from(id: u64, grid: Grid) -> Tile {
        Tile {
            id,
            edges: grid.get_edges(),
            grid,
        }
    }

    fn rotate(&self, n: usize) -> Tile {
        (0..n).fold(self.clone(), |t, _| Tile::from(t.id, t.grid.rotate()))
    }

    fn flip(&self) -> Tile {
        Tile::from(self.id, self.grid.flip())
    }
}

impl Coord {
    fn from(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    fn add(&self, other: Coord) -> Coord {
        Coord::from(self.x + other.x, self.y + other.y)
    }
}

trait GridTools {
    fn get_edges(&self) -> Vec<String>;
    fn strip(&self) -> Grid;
    fn rotate(&self) -> Grid;
    fn flip(&self) -> Grid;
    fn get_size(&self) -> Size;
}

impl GridTools for Grid {
    fn get_edges(&self) -> Vec<String> {
        vec![
            self.iter().next().unwrap().iter().collect(),
            self.iter().map(|l| l.iter().last().unwrap()).collect(),
            self.iter().last().unwrap().iter().rev().collect(),
            self.iter()
                .map(|l| l.iter().next().unwrap())
                .rev()
                .collect(),
        ]
    }

    fn strip(&self) -> Grid {
        let size = self.get_size();
        self.iter()
            .skip(1)
            .take(size.height - 2)
            .map(|r| r.iter().skip(1).take(size.width - 2).cloned().collect())
            .collect()
    }

    fn rotate(&self) -> Grid {
        let mut next_grid = self.clone();
        let mut next_x = self.get_size().height;

        for (_, r) in self.iter().enumerate() {
            next_x -= 1;
            for (x, c) in r.iter().enumerate() {
                let next_y = x;
                next_grid[next_y][next_x] = *c
            }
        }

        next_grid
    }

    fn flip(&self) -> Grid {
        self.iter()
            .map(|row| row.iter().cloned().rev().collect())
            .collect()
    }

    fn get_size(&self) -> Size {
        Size {
            width: self.iter().next().unwrap().len(),
            height: self.len(),
        }
    }
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .trim()
        .split("\n\n")
        .map(|c| {
            let id = c
                .lines()
                .next()
                .unwrap()
                .trim_start_matches("Tile ")
                .trim_end_matches(':')
                .parse()
                .unwrap();

            let grid: Grid = c.lines().skip(1).map(|l| l.chars().collect()).collect();
            let edges = grid.get_edges();

            Tile { id, edges, grid }
        })
        .collect()
}
