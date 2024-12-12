use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

enum RuleResult {
	Single(u64),
	Double((u64, u64)),
}

fn rules(val: u64) -> RuleResult {
	let s = val.to_string();
	if val == 0 {
		return RuleResult::Single(1);
	} else if s.len() % 2 == 0 {
		// There's probably a way to do this with math but I'm dumb
		let mid = s.len() / 2;
		let v1 = &s[..mid];
		let v2 = &s[mid..];
		return RuleResult::Double((v1.parse().unwrap(), v2.parse().unwrap()));
	}
	return RuleResult::Single(val * 2024);
}

fn blink(val: u64, blink_num: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
	let key = (val, blink_num);
	// If we've seen this stone value at this blink level before we already know what the resulting stone count will be.
	if let Some(&v) = cache.get(&key) {
		return v;
	}
	let res = rules(val);
	if blink_num == 1 {
		// On the final blink we don't care what the values are.
		// The question only asks for what the total number of stones are.
		return match res {
			RuleResult::Single(_) => 1,
			RuleResult::Double(_) => 2,
		};
	}
	match res {
		RuleResult::Single(s) => {
			let result = blink(s, blink_num - 1, cache);
			cache.insert(key, result);
			result
		}
		RuleResult::Double(d) => {
			let result = blink(d.0, blink_num - 1, cache) + blink(d.1, blink_num - 1, cache);
			cache.insert(key, result);
			result
		}
	}
}

fn run(count: usize, stones: &[u64]) -> u64 {
	// Cache the stone value, blink level, and resulting stone count generated from said value
	let mut cache = HashMap::new();
	// The stones don't actually interact with eachother.
	// The "order" mentioned in the problem doesn't matter. It's misleading.
	// We can calculate the result of each stone on its own by running the blinks through it one by one.
	stones.iter().map(|s| blink(*s, count, &mut cache)).sum()
}

fn main() {
	let mut file = File::open("./input.txt").unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s).unwrap();
	let stones: Vec<u64> = s.split(" ").map(|e| e.trim().parse().unwrap()).collect();
	println!("{}", run(25, &stones));
	println!("{}", run(75, &stones));
}
