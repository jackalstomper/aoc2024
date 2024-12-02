use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {}

#[test]
fn part1() {
	let file = File::open("./input.txt").unwrap();
	let mut left = Vec::new();
	let mut right = Vec::new();
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		let mut i = l.split("   ").map(|c| c.parse::<i32>().unwrap());
		left.push(i.next().unwrap());
		right.push(i.next().unwrap());
	}
	left.sort();
	right.sort();
	let mut sum = 0;
	for (i, e) in left.iter().enumerate() {
		sum += (right[i] - e).abs();
	}

	println!("Sum: {}", sum);
}

#[test]
fn part2() {
	let file = File::open("./input.txt").unwrap();
	let mut counts = HashMap::new();
	let mut left_list = Vec::new();
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		let mut i = l.split("   ").map(|c| c.parse::<i32>().unwrap());
		let left = i.next().unwrap();
		let right = i.next().unwrap();
		let count = match counts.get(&right) {
			Some(e) => *e,
			None => 0,
		};
		counts.insert(right, count + 1);
		left_list.push(left);
	}
	let mut sum = 0;
	for left in left_list {
		let count = match counts.get(&left) {
			Some(e) => *e,
			None => 0,
		};
		sum += left * count;
	}

	println!("Sum: {}", sum);
}
