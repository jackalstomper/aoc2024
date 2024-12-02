use std::fs::File;
use std::io::{self, BufRead};

fn main() {}

fn is_valid(row: &[i32]) -> bool {
	let mut asc = true;
	let mut desc = true;
	for i in 1..row.len() {
		let left = row[i - 1];
		let right = row[i];
		if left >= right {
			asc = false;
		}
		if left <= right {
			desc = false;
		}
		if (right - left).abs() > 3 {
			return false;
		}
	}

	asc || desc
}

fn is_valid_part2(row: &[i32]) -> bool {
	if is_valid(row) {
		return true;
	}

	// brute force it.
	// try removing each element until we find something that works
	for i in 0..row.len() {
		let s = [&row[..i], &row[i + 1..]].concat();
		if is_valid(&s) {
			return true;
		}
	}
	false
}

#[test]
fn part1() {
	let file = File::open("./input.txt").unwrap();
	let mut sum = 0;
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		let row: Vec<i32> = l.split(" ").map(|c| c.parse().unwrap()).collect();
		if is_valid(&row) {
			sum += 1;
		}
	}
	println!("Sum is {}", sum);
}

#[test]
fn part2() {
	let file = File::open("./input.txt").unwrap();
	let mut sum = 0;
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		let row: Vec<i32> = l.split(" ").map(|c| c.parse().unwrap()).collect();
		if is_valid_part2(&row) {
			sum += 1;
		}
	}
	println!("Sum is {}", sum);
}
