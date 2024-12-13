use std::fs::File;
use std::io::{self, BufRead};

type Int = i64;
const MULTIPLIER: Int = 10000000000000;

#[derive(Default)]
struct Game {
	a: (Int, Int),
	b: (Int, Int),
	p: (Int, Int),
}

impl Game {
	fn parse(file: File) -> Vec<Self> {
		let mut games = Vec::new();
		let mut game = Self::default();
		for line in io::BufReader::new(file).lines() {
			let l = line.unwrap();
			if l.is_empty() {
				games.push(game);
				game = Self::default();
				continue;
			}
			let parts: Vec<&str> = l.split(":").collect();
			let cords: Vec<&str> = parts[1].split(",").collect();
			let cordsplit = if parts[0] == "Prize" { "=" } else { "+" };
			let xv: Int = cords[0].split(cordsplit).nth(1).unwrap().parse().unwrap();
			let yv: Int = cords[1].split(cordsplit).nth(1).unwrap().parse().unwrap();
			match parts[0] {
				"Button A" => {
					game.a = (xv, yv);
				}
				"Button B" => {
					game.b = (xv, yv);
				}
				"Prize" => {
					game.p = (xv, yv);
				}
				_ => panic!("invalid input"),
			}
		}
		games.push(game);
		games
	}

	fn solve(&self, p2: bool) -> Int {
		let (ax, ay) = self.a;
		let (bx, by) = self.b;
		let (px, py) = if p2 {
			(self.p.0 + MULTIPLIER, self.p.1 + MULTIPLIER)
		} else {
			self.p
		};
		let det = (ax * by) - (ay * bx);
		let bp = by * px - bx * py;
		let ap = ax * py - ay * px;
		if (ap * 3) % det == 0 && bp % det == 0 {
			return (ap + bp * 3) / det;
		}
		0
	}
}

fn main() {
	let file = File::open("./input.txt").unwrap();
	let games = Game::parse(file);
	let p1: Int = games.iter().map(|g| g.solve(false)).sum();
	let p2: Int = games.iter().map(|g| g.solve(true)).sum();
	println!("{}", p1);
	println!("{}", p2);
}
