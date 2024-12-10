use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const DIRS: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Dir {
	Up,
	Down,
	Left,
	Right,
}

impl Dir {
	fn opposite(&self) -> Dir {
		match self {
			Dir::Up => Dir::Down,
			Dir::Down => Dir::Up,
			Dir::Left => Dir::Right,
			Dir::Right => Dir::Left,
		}
	}
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Pos {
	x: i32,
	y: i32,
}

impl Pos {
	fn shift(mut self, dir: Dir, amt: i32) -> Pos {
		match dir {
			Dir::Up => self.y -= amt,
			Dir::Down => self.y += amt,
			Dir::Left => self.x -= amt,
			Dir::Right => self.x += amt,
		}
		self
	}
}

struct Grid {
	tiles: Vec<Vec<u8>>,
}

impl Grid {
	fn parse(file: File) -> Self {
		let mut tiles: Vec<Vec<u8>> = Vec::new();
		for line in io::BufReader::new(file).lines() {
			let l = line.unwrap();
			let u = l
				.chars()
				.map(|c| {
					if c == '.' {
						0xff // for testing using the example input
					} else {
						c.to_digit(10).unwrap() as u8
					}
				})
				.collect();
			tiles.push(u);
		}
		Self { tiles }
	}

	fn get(&self, pos: Pos) -> Option<u8> {
		if pos.y < 0 || pos.x < 0 {
			return None;
		}
		let row = self.tiles.get(pos.y as usize)?;
		row.get(pos.x as usize).cloned()
	}
}

fn main() {
	let file = File::open("./input.txt").unwrap();
	let grid = Grid::parse(file);
	println!("{}", search_paths(&grid, false));
	println!("{}", search_paths(&grid, true));
}

fn search_paths(grid: &Grid, p2: bool) -> usize {
	let mut sum = 0;
	for y in 0..grid.tiles.len() {
		for x in 0..grid.tiles[0].len() {
			if grid.tiles[y][x] != 0 {
				continue;
			}
			let p = Pos {
				y: y as i32,
				x: x as i32,
			};
			sum += search(&grid, p, 1, p2);
		}
	}
	sum
}

fn search(grid: &Grid, pos: Pos, val: u8, p2: bool) -> usize {
	let mut nines = HashSet::new();
	let mut path_count = 0;
	for dir in DIRS {
		let mut stack = vec![(pos.shift(dir, 1), dir, val)];
		while !stack.is_empty() {
			let (p, d, v) = stack.pop().unwrap();
			let Some(i) = grid.get(p) else {
				continue;
			};
			if i != v {
				continue;
			}
			if v == 9 {
				if p2 {
					path_count += 1;
				} else {
					nines.insert(p);
				}
				continue;
			}
			let opp = d.opposite();
			for dd in DIRS {
				if dd == opp {
					continue; // dont go back the way we came
				}
				stack.push((p.shift(dd, 1), dd, v + 1));
			}
		}
	}
	if p2 {
		path_count
	} else {
		nines.len()
	}
}
