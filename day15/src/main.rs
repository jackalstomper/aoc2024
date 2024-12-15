use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos {
	x: i32,
	y: i32,
}

impl Pos {
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	fn shift(mut self, dir: Dir) -> Pos {
		match dir {
			Dir::Up => self.y -= 1,
			Dir::Right => self.x += 1,
			Dir::Down => self.y += 1,
			Dir::Left => self.x -= 1,
		}
		self
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
	Up,
	Right,
	Down,
	Left,
}

struct Map {
	tiles: Vec<Vec<char>>,
	robot: Pos,
}

impl Map {
	fn parse(file: File) -> (Self, Vec<Dir>) {
		let mut parse_dirs = false;
		let mut tiles = Vec::new();
		let mut dirs = Vec::new();
		let mut robot = Pos::new(0, 0);
		for (y, line) in io::BufReader::new(file)
			.lines()
			.map(|l| l.unwrap())
			.enumerate()
		{
			if line.is_empty() {
				parse_dirs = true;
				continue;
			}
			if parse_dirs {
				for c in line.chars() {
					let d = match c {
						'^' => Dir::Up,
						'>' => Dir::Right,
						'v' => Dir::Down,
						'<' => Dir::Left,
						_ => panic!("invalid input"),
					};
					dirs.push(d);
				}
			} else {
				let row = line
					.chars()
					.enumerate()
					.map(|(x, c)| {
						let pos = Pos::new(x as i32, y as i32);
						if c == '@' {
							robot = pos;
						}
						c
					})
					.collect();
				tiles.push(row);
			}
		}

		(Self { tiles, robot }, dirs)
	}

	fn get(&self, pos: Pos) -> Option<char> {
		let row = self.tiles.get(pos.y as usize)?;
		row.get(pos.x as usize).cloned()
	}

	fn put(&mut self, pos: Pos, tile: char) {
		self.tiles[pos.y as usize][pos.x as usize] = tile;
	}

	fn swap(&mut self, src: Pos, dst: Pos) {
		let tmp = self.get(src).unwrap();
		self.put(src, self.get(dst).unwrap());
		self.put(dst, tmp);
	}

	/// Returns the pos src should move to
	fn move_to(&mut self, src: Pos, dst: Pos, dir: Dir) -> bool {
		let Some(dst_tile) = self.get(dst) else {
			return false;
		};
		match dst_tile {
			'.' => {
				self.swap(src, dst);
				return true;
			}
			'#' => return false,
			'O' => {
				if self.move_to(dst, dst.shift(dir), dir) {
					self.swap(src, dst);
					return true;
				}
			}
			_ => panic!("invalid state"),
		};
		false
	}

	fn gps(&self, pos: Pos) -> i32 {
		(100 * pos.y) + pos.x
	}

	fn print(&self) {
		for row in &self.tiles {
			for col in row {
				print!("{}", col);
			}
			println!("");
		}
	}
}

fn main() {
	let file = File::open("./input.txt").unwrap();
	let (mut map, dirs) = Map::parse(file);

	for dir in dirs {
		let new_pos = map.robot.shift(dir);
		if map.move_to(map.robot, new_pos, dir) {
			map.robot = new_pos;
		}
	}
	map.print();

	let mut sum = 0;
	for (y, row) in map.tiles.iter().enumerate() {
		for (x, tile) in row.iter().enumerate() {
			if *tile == 'O' {
				let gps = 100 * y + x;
				sum += gps;
			}
		}
	}
	println!("{}", sum);
}
