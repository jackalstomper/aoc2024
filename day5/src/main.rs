use std::fs::File;
use std::io::{self, BufRead};

fn main() {}

fn get_idx(row: &Vec<i32>, rule: &(i32, i32)) -> Option<(usize, usize)> {
	let Some(bi) = row.iter().position(|&e| e == rule.0) else {
		return None;
	};
	let Some(ai) = row.iter().position(|&e| e == rule.1) else {
		return None;
	};
	Some((bi, ai))
}

struct Instructions {
	rules: Vec<(i32, i32)>,
	pages: Vec<Vec<i32>>,
}

impl Instructions {
	fn new(file: File) -> Self {
		let mut rules = Vec::new();
		let mut pages = Vec::new();
		let mut parse_pages = false;
		for line in io::BufReader::new(file).lines() {
			let l = line.unwrap();
			if l.is_empty() {
				// pages to produce are next
				parse_pages = true;
				continue;
			}
			if parse_pages {
				let v: Vec<i32> = l.split(",").map(|d| d.parse().unwrap()).collect();
				pages.push(v);
			} else {
				let mut s = l.split("|");
				let left = s.next().unwrap().parse().unwrap();
				let right = s.next().unwrap().parse().unwrap();
				rules.push((left, right));
			}
		}

		Self { rules, pages }
	}

	fn is_valid(&self, row_idx: usize) -> bool {
		let row = &self.pages[row_idx];
		for rule in &self.rules {
			let Some((bi, ai)) = get_idx(row, rule) else {
				continue;
			};
			if bi > ai {
				return false;
			}
		}
		true
	}

	/// Swap each pair of elements that don't conform to a rule until we have a valid list
	fn fix_pages(&mut self, row_idx: usize) {
		let row = &mut self.pages[row_idx];
		loop {
			let mut fixed_something = false;
			for rule in &self.rules {
				let Some((bi, ai)) = get_idx(row, rule) else {
					continue;
				};
				if bi > ai {
					// swap the elements
					let tmp = row[bi];
					row[bi] = row[ai];
					row[ai] = tmp;
					fixed_something = true;
				}
			}
			if !fixed_something {
				break; // We're ordered
			}
		}
	}
}

#[test]
fn part1() {
	let file = File::open("./input.txt").unwrap();
	let inst = Instructions::new(file);
	let mut sum = 0;
	for i in 0..inst.pages.len() {
		if inst.is_valid(i) {
			let mid_idx = inst.pages[i].len() / 2;
			sum += inst.pages[i][mid_idx];
		}
	}
	println!("Sum is {}", sum);
}

#[test]
fn part2() {
	let file = File::open("./input.txt").unwrap();
	let mut inst = Instructions::new(file);
	let mut sum = 0;
	for i in 0..inst.pages.len() {
		if !inst.is_valid(i) {
			inst.fix_pages(i);
			let mid_idx = inst.pages[i].len() / 2;
			sum += inst.pages[i][mid_idx];
		}
	}
	println!("Sum is {}", sum);
}
