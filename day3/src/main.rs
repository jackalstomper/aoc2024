use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {}

#[test]
fn part1() {
	let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
	let file = File::open("./input.txt").unwrap();
	let mut sum = 0;
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		for cap in re.captures_iter(&l) {
			let l: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
			let r: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
			sum += l * r;
		}
	}
	println!("Sum is {}", sum);
}

#[test]
fn part2() {
	let re = Regex::new(r"(?:do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();
	let file = File::open("./input.txt").unwrap();
	let mut sum = 0;
	let mut enabled = true;
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		for cap in re.captures_iter(&l) {
			let str = cap.get(0).unwrap().as_str();
			if str == "don't()" {
				enabled = false;
			} else if str == "do()" {
				enabled = true;
			} else if enabled {
				let l: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
				let r: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
				sum += l * r;
			}
		}
	}
	println!("Sum is {}", sum);
}
