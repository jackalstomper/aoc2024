use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos {
	x: i32,
	y: i32,
}

impl Pos {
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	fn diff(&self, rhs: &Self) -> Self {
		let x_diff = self.x - rhs.x;
		let y_diff = self.y - rhs.y;
		Pos::new(x_diff, y_diff)
	}
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Node {
	ty: char,
	pos: Pos,
}

impl Node {
	fn new(ty: char, x: usize, y: usize) -> Self {
		Self {
			ty,
			pos: Pos::new(x as i32, y as i32),
		}
	}

	fn p1_antinode(&self, other: &Self) -> Pos {
		let diff = self.pos.diff(&other.pos);
		Pos::new(self.pos.x + diff.x, self.pos.y + diff.y)
	}
}

struct Map {
	nodes: Vec<Node>,
	x_limit: i32,
	y_limit: i32,
}

impl Map {
	fn p2_antinodes(&self, li: usize, ri: usize) -> Vec<Pos> {
		let mut out = Vec::new();
		let diff = self.nodes[li].pos.diff(&self.nodes[ri].pos);
		for op in [0, 1] {
			let mut i = self.nodes[li].pos;
			while self.in_limit(&i) {
				out.push(i);
				if op == 0 {
					i.x += diff.x;
					i.y += diff.y;
				} else {
					i.x -= diff.x;
					i.y -= diff.y;
				}
			}
		}
		out
	}

	fn in_limit(&self, pos: &Pos) -> bool {
		pos.x >= 0 && pos.x < self.x_limit && pos.y >= 0 && pos.y < self.y_limit
	}

	fn new(file: File) -> Self {
		let mut nodes = Vec::new();
		let lines: Vec<String> = io::BufReader::new(file)
			.lines()
			.map(|l| l.unwrap())
			.collect();

		let y_limit = lines.len();
		let mut x_limit = 0;
		for (y, l) in lines.iter().enumerate() {
			x_limit = l.len();
			for (x, c) in l.chars().enumerate() {
				if c != '.' {
					nodes.push(Node::new(c, x, y));
				}
			}
		}
		Self {
			nodes,
			y_limit: y_limit as i32,
			x_limit: x_limit as i32,
		}
	}
}

fn main() {
	let file = File::open("./input.txt").unwrap();
	let map = Map::new(file);
	p1(&map);
	p2(&map);
}

fn p1(map: &Map) {
	let mut antis = HashSet::new();
	for i in 0..map.nodes.len() {
		for j in i + 1..map.nodes.len() {
			let n1 = &map.nodes[i];
			let n2 = &map.nodes[j];
			if n1.ty == n2.ty {
				let aa = [n1.p1_antinode(n2), n2.p1_antinode(n1)];
				for a in aa {
					if map.in_limit(&a) {
						antis.insert(a);
					}
				}
			}
		}
	}
	println!("{}", antis.len());
}

fn p2(map: &Map) {
	let mut antis = HashSet::new();
	for i in 0..map.nodes.len() {
		for j in i + 1..map.nodes.len() {
			if map.nodes[i].ty == map.nodes[j].ty {
				for a in map.p2_antinodes(i, j) {
					antis.insert(a);
				}
			}
		}
	}
	println!("{}", antis.len());
}
