use std::fs::File;
use std::io::{self, BufRead};
use std::string;

fn main() {}

const P1_TARGET: [char; 4] = ['X', 'M', 'A', 'S'];

enum Dir {
	Up,
	Down,
	Left,
	Right,
	UpLeft,
	UpRight,
	DownLeft,
	DownRight,
}

impl Dir {
	fn offset(&self, x: i32, y: i32) -> (i32, i32) {
		match self {
			Dir::Up => (x, y - 1),
			Dir::Down => (x, y + 1),
			Dir::Left => (x - 1, y),
			Dir::Right => (x + 1, y),
			Dir::UpLeft => (x - 1, y - 1),
			Dir::UpRight => (x + 1, y - 1),
			Dir::DownLeft => (x - 1, y + 1),
			Dir::DownRight => (x + 1, y + 1),
		}
	}
}

const DIRS: [Dir; 8] = [
	Dir::Up,
	Dir::Down,
	Dir::Left,
	Dir::Right,
	Dir::UpLeft,
	Dir::UpRight,
	Dir::DownLeft,
	Dir::DownRight,
];

struct Grid {
	vec: Vec<String>,
}

impl Grid {
	fn new(vec: Vec<String>) -> Self {
		Self { vec }
	}

	fn get(&self, x: i32, y: i32) -> Option<char> {
		if x < 0 || y < 0 {
			return None;
		}
		self
			.vec
			.get(y as usize)
			.and_then(|e| e.chars().nth(x as usize))
	}

	fn y_len(&self) -> i32 {
		self.vec.len() as i32
	}

	fn x_len(&self) -> i32 {
		self.vec[0].len() as i32
	}

	fn find_xmas(&self, mut x: i32, mut y: i32, dir: Dir) -> bool {
		for i in 0..P1_TARGET.len() {
			let Some(c) = self.get(x, y) else {
				return false;
			};
			if c != P1_TARGET[i] {
				return false;
			}
			(x, y) = dir.offset(x, y);
		}
		true
	}
}

#[test]
fn part1() {
	let file = File::open("./input.txt").unwrap();
	let mut rows = Vec::new();
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		rows.push(l);
	}

	let grid = Grid::new(rows);
	let mut sum = 0;
	for y in 0..grid.y_len() {
		for x in 0..grid.x_len() {
			let Some(c) = grid.get(x, y) else {
				continue;
			};
			if c == 'X' {
				for dir in DIRS {
					if grid.find_xmas(x, y, dir) {
						sum += 1
					}
				}
			}
		}
	}
	println!("Sum is {}", sum);
}

#[test]
fn part2() {
	let file = File::open("./input.txt").unwrap();
	let mut rows = Vec::new();
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		rows.push(l);
	}

	let grid = Grid::new(rows);
	let mut sum = 0;
	for y in 1..grid.y_len() - 1 {
		for x in 1..grid.x_len() - 1 {
			let Some(c) = grid.get(x, y) else {
				continue;
			};
			if c != 'A' {
				continue;
			}
			let Some(ul) = grid.get(x - 1, y - 1) else {
				continue;
			};
			let Some(ur) = grid.get(x + 1, y - 1) else {
				continue;
			};
			let Some(dl) = grid.get(x - 1, y + 1) else {
				continue;
			};
			let Some(dr) = grid.get(x + 1, y + 1) else {
				continue;
			};
			let ul_valid = (ul == 'M' && dr == 'S') || (ul == 'S' && dr == 'M');
			let ur_valid = (ur == 'M' && dl == 'S') || (ur == 'S' && dl == 'M');
			if ul_valid && ur_valid {
				sum += 1;
			}
		}
	}
	println!("Sum is {}", sum);
}
