use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
	Up,
	Down,
	Left,
	Right,
}

impl Dir {
	fn rotate(&mut self) {
		*self = match self {
			Dir::Up => Dir::Right,
			Dir::Down => Dir::Left,
			Dir::Left => Dir::Up,
			Dir::Right => Dir::Down,
		}
	}
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Guard {
	dir: Dir,
	pos: Pos,
}

struct Map {
	// It's either a block or it's not. bool is sufficient.
	tiles: Vec<Vec<bool>>,
}

impl Map {
	fn parse(file: File) -> (Map, Guard) {
		let mut guard = Guard {
			dir: Dir::Up,
			pos: Pos { x: 0, y: 0 },
		};
		let tiles = io::BufReader::new(file)
			.lines()
			.map(|l| l.unwrap())
			.enumerate()
			.map(|(y, l)| {
				l.chars()
					.enumerate()
					.map(|(x, b)| {
						if b == '^' {
							guard.pos.x = x as i32;
							guard.pos.y = y as i32;
							false
						} else {
							b == '#'
						}
					})
					.collect()
			})
			.collect();

		(Map { tiles }, guard)
	}

	fn get(&self, pos: Pos) -> Option<bool> {
		if pos.x < 0 || pos.y < 0 {
			return None;
		}
		let row = self.tiles.get(pos.y as usize)?;
		row.get(pos.x as usize).cloned()
	}

	fn put(&mut self, pos: Pos, block: bool) -> Result<(), i32> {
		if pos.x < 0 || pos.y < 0 {
			return Err(1);
		}
		if pos.y as usize >= self.tiles.len() {
			return Err(2);
		}
		let row = &mut self.tiles[pos.y as usize];
		if pos.x as usize >= row.len() {
			return Err(3);
		}
		row[pos.x as usize] = block;
		Ok(())
	}
}

fn main() {
	let file = File::open("./input.txt").unwrap();
	let (mut map, guard) = Map::parse(file);
	println!("{}", p1(&map, guard));
	println!("{}", p2(&mut map, guard));
}

fn p1(map: &Map, mut guard: Guard) -> usize {
	let mut visited = HashSet::new();
	visited.insert(guard.pos);
	loop {
		let next_pos = guard.pos.shift(guard.dir, 1);
		let Some(tile) = map.get(next_pos) else {
			break;
		};
		if tile {
			guard.dir.rotate();
		} else {
			guard.pos = next_pos;
			visited.insert(next_pos);
		}
	}
	visited.len()
}

fn p2(map: &mut Map, mut guard: Guard) -> u32 {
	let mut visited = HashSet::new();
	let mut barrier_count = 0;
	loop {
		let next_pos = guard.pos.shift(guard.dir, 1);
		let Some(tile) = map.get(next_pos) else {
			break;
		};
		if tile {
			guard.dir.rotate();
		} else {
			if !visited.contains(&next_pos) {
				map.put(next_pos, true).unwrap();
				if is_loop(map, guard) {
					barrier_count += 1;
				}
				map.put(next_pos, false).unwrap();
			}
			guard.pos = next_pos;
			visited.insert(next_pos);
		}
	}

	barrier_count
}

fn is_loop(map: &Map, mut guard: Guard) -> bool {
	let mut turns = HashSet::new();
	loop {
		let next_pos = guard.pos.shift(guard.dir, 1);
		let Some(tile) = map.get(next_pos) else {
			break;
		};
		if tile {
			guard.dir.rotate();
			if turns.contains(&guard) {
				return true;
			}
			turns.insert(guard);
		} else {
			guard.pos = next_pos;
		}
	}
	false
}
