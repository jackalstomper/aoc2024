use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;

const HEIGHT: i32 = 103;
const WIDTH: i32 = 101;
const MID_H: i32 = (HEIGHT - 1) / 2;
const MID_W: i32 = (WIDTH - 1) / 2;
const SQUARE_SIZE: i32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Vec2D {
	x: i32,
	y: i32,
}

impl Vec2D {
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}
}

impl Add for Vec2D {
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self.x += rhs.x;
		self.y += rhs.y;
		self
	}
}

#[derive(Clone, Copy)]
struct Robot {
	pos: Vec2D,
	vel: Vec2D,
}

impl Robot {
	fn new(pos: Vec2D, vel: Vec2D) -> Self {
		Self { pos, vel }
	}

	fn tick(&mut self) {
		let mut new = self.pos + self.vel;
		if new.x >= WIDTH {
			new.x = new.x - WIDTH;
		}
		if new.x < 0 {
			new.x = WIDTH + new.x;
		}
		if new.y >= HEIGHT {
			new.y = new.y - HEIGHT;
		}
		if new.y < 0 {
			new.y = HEIGHT + new.y;
		}
		self.pos = new;
	}

	fn is_mid(&self) -> bool {
		self.pos.x == MID_W || self.pos.y == MID_H
	}

	fn quad_id(&self) -> u8 {
		if self.is_mid() {
			return 0xff;
		}
		let x = if self.pos.x < MID_W { 1 << 3 } else { 1 << 2 };
		let y = if self.pos.y < MID_H { 1 << 1 } else { 1 };
		x | y
	}
}

fn main() {
	let file = File::open("./input.txt").unwrap();
	let robots: Vec<Robot> = io::BufReader::new(file)
		.lines()
		.map(|line| {
			let l = line.unwrap();
			let [p1, p2, v1, v2]: [i32; 4] = l
				.split(" ")
				.map(|s| s.split_once(",").unwrap())
				.flat_map(|s| [s.0[2..].parse().unwrap(), s.1.parse().unwrap()])
				.collect::<Vec<i32>>()
				.try_into()
				.unwrap();
			Robot::new(Vec2D::new(p1, p2), Vec2D::new(v1, v2))
		})
		.collect();
	p1(robots.clone());
	p2(robots.clone());
}

fn p1(mut robots: Vec<Robot>) {
	for _ in 0..100 {
		for robot in &mut robots {
			robot.tick();
		}
	}
	let mut quads: HashMap<u8, i32> = HashMap::new();
	for robot in robots {
		let id = robot.quad_id();
		if id != 0xff {
			quads
				.entry(robot.quad_id())
				.and_modify(|c| *c += 1)
				.or_insert(1);
		}
	}
	let sum = quads.values().fold(1, |acc, c| acc * c);
	println!("{}", sum);
}

fn p2(mut robots: Vec<Robot>) {
	// Look for a block of robots, good indicator that we found it.
	for i in 0..10_000 {
		for robot in &mut robots {
			robot.tick();
		}

		let mut ps = HashSet::new();
		for robot in &robots {
			ps.insert(robot.pos);
		}

		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				let mut good = true;
				'outer: for xx in x..x + SQUARE_SIZE {
					for yy in y..y + SQUARE_SIZE {
						let p = Vec2D::new(xx, yy);
						if !ps.contains(&p) {
							good = false;
							break 'outer;
						}
					}
				}

				if good {
					print_map(&robots);
					println!("Easter egg shows up at {} iterations", i + 1);
					return;
				}
			}
		}
	}
}

fn print_map(robots: &[Robot]) {
	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			let p = Vec2D { x, y };
			let r_count = robots.iter().filter(|r| r.pos == p).count();
			if r_count > 0 {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!("");
	}
}
