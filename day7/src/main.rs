use std::fs::File;
use std::io::{self, BufRead};

fn main() {}

enum Op {
	Mul,
	Sum,
	Concat,
}

impl Op {
	fn operate(&self, l: u64, r: u64) -> u64 {
		match self {
			Op::Mul => l * r,
			Op::Sum => l + r,
			Op::Concat => {
				let times = if r == 0 { 1 } else { 10u64.pow(r.ilog10() + 1) };
				l * times + r
			}
		}
	}
}

/// Strict left to right operations and only needing to find one working solution means dfs can be used.
///
/// Example with the input "292: 11 6 16 20" (incorrect branches are omitted)
///
/// 		11
/// 		| +6
/// 		17
///			| *16
///			272
/// 		| +20
/// 		292
///
/// Once we reach the branch that computes to 292 we've found our solution and can return true back up the tree.  
/// If the branch is invalid return false
///
/// p2 functions the same way but with 3 branches for the concat operator instead of two
fn dfs(result: u64, values: &[u64], ops: &[Op], progress: u64) -> bool {
	if progress > result {
		return false;
	}
	if values.len() == 1 {
		ops.iter().any(|o| {
			let p = o.operate(progress, values[0]);
			result == p
		})
	} else {
		ops.iter().any(|o| {
			let p = o.operate(progress, values[0]);
			dfs(result, &values[1..], ops, p)
		})
	}
}

struct Input {
	result: u64,
	values: Vec<u64>,
}

fn parse(file: File) -> Vec<Input> {
	io::BufReader::new(file)
		.lines()
		.map(|line| {
			let l = line.unwrap();
			let sl: Vec<&str> = l.split(":").collect();
			let result: u64 = sl[0].parse().unwrap();
			let values: Vec<u64> = sl[1]
				.split(" ")
				.filter(|e| !e.is_empty())
				.map(|e| e.parse().unwrap())
				.collect();

			Input { result, values }
		})
		.collect()
}

#[test]
fn part1() {
	let file = File::open("./input.txt").unwrap();
	let inputs = parse(file);
	let mut sum = 0u64;
	let ops = [Op::Sum, Op::Mul];
	for input in inputs {
		let result = input.result;
		let values = &input.values;
		if dfs(result, &values[1..], &ops, values[0]) {
			sum += result;
		}
	}
	println!("Sum is {}", sum);
}

#[test]
fn part2() {
	let file = File::open("./input.txt").unwrap();
	let inputs = parse(file);
	let mut sum = 0u64;
	let ops = [Op::Sum, Op::Mul, Op::Concat];
	for input in inputs {
		let result = input.result;
		let values = &input.values;
		if dfs(result, &values[1..], &ops, values[0]) {
			sum += result;
		}
	}
	println!("Sum is {}", sum);
}
