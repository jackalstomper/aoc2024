use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const DIRS: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];

const CORNERS: [[Dir; 2]; 4] = [
	[Dir::Up, Dir::Left],
	[Dir::Up, Dir::Right],
	[Dir::Down, Dir::Left],
	[Dir::Down, Dir::Right],
];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
	x: i32,
	y: i32,
}

impl Pos {
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	fn shift(mut self, dir: Dir, amt: i32) -> Self {
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
		let tiles: Vec<Vec<u8>> = io::BufReader::new(file)
			.lines()
			.map(|l| l.unwrap().bytes().collect())
			.collect();
		Self { tiles }
	}

	fn size(&self) -> usize {
		self.tiles.len() * self.tiles[0].len()
	}

	fn get(&self, pos: Pos) -> Option<u8> {
		let row = self.tiles.get(pos.y as usize)?;
		row.get(pos.x as usize).copied()
	}

	fn get_regions(&self) -> Vec<Region> {
		let mut out: Vec<Region> = Vec::new();
		for y in 0..self.tiles.len() {
			for x in 0..self.tiles[y].len() {
				let p = Pos::new(x as i32, y as i32);
				if out.iter().any(|r| r.tiles.contains(&p)) {
					continue;
				}
				let region = Region::search(self, p, self.tiles[y][x]);
				out.push(region);
			}
		}
		out
	}

	fn get_corners(&self, pos: Pos, target: u8) -> i32 {
		let mut corners = 0;
		for corner in CORNERS {
			let a = pos.shift(corner[0], 1); // "up"
			let b = pos.shift(corner[1], 1); // "right"
			let c = pos.shift(corner[0], 1).shift(corner[1], 1); // "upright"
			let am = self.get(a).is_some_and(|u| u == target);
			let bm = self.get(b).is_some_and(|u| u == target);
			let cm = self.get(c).is_some_and(|u| u == target);

			if !am && !bm {
				// convex corner (clockwise)
				// -+
				//  |
				corners += 1;
			} else if am && bm && !cm {
				// concave corner (clockwise)
				// |
				// +-
				corners += 1;
			}
		}
		corners
	}
}

struct Region {
	target: u8,
	tiles: HashSet<Pos>,
	perimeter: i32,
	corners: i32,
}

impl Region {
	fn search(grid: &Grid, pos: Pos, target: u8) -> Self {
		let mut tiles = HashSet::new();
		tiles.insert(pos);
		let mut perimeters = Vec::with_capacity(grid.size());
		let mut stack: Vec<(Pos, Dir)> = DIRS.iter().map(|&d| (pos.shift(d, 1), d)).collect();
		while !stack.is_empty() {
			let (p, d) = stack.pop().unwrap();
			let Some(i) = grid.get(p) else {
				perimeters.push(p);
				continue;
			};
			if i != target {
				perimeters.push(p);
				continue;
			}
			if tiles.contains(&p) {
				continue; // DRY
			}
			tiles.insert(p);
			let opp = d.opposite();
			for dd in DIRS {
				if opp == dd {
					continue; // Don't go back where we came from
				}
				stack.push((p.shift(dd, 1), dd));
			}
		}

		let corners = tiles.iter().map(|&t| grid.get_corners(t, target)).sum();
		Self {
			target,
			tiles,
			perimeter: perimeters.len() as i32,
			corners,
		}
	}

	fn perimeter(&self) -> i32 {
		self.perimeter
	}

	fn area(&self) -> i32 {
		self.tiles.len() as i32
	}

	fn bulk(&self) -> i32 {
		self.corners
	}
}

fn main() {
	let file = File::open("./input.txt").unwrap();
	let grid = Grid::parse(file);
	let regions = grid.get_regions();
	println!("{}", p1(&regions));
	println!("{}", p2(&regions));
}

fn p1(regions: &[Region]) -> i32 {
	regions.iter().map(|r| r.area() * r.perimeter()).sum()
}

fn p2(regions: &[Region]) -> i32 {
	regions.iter().map(|r| r.area() * r.bulk()).sum()
}
