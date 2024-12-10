use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone, PartialEq, Eq)]
struct AocFile {
	id: i32,
	size: i32,
}

fn p2_parse(file: File) -> Vec<AocFile> {
	let mut files = Vec::new();
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		let mut free_space = false;
		let mut file_id = 0;
		for c in l.chars() {
			let size = c.to_digit(10).unwrap() as i32;
			if !free_space {
				files.push(AocFile {
					id: file_id,
					size: size,
				});
				file_id += 1;
			} else if size != 0 {
				files.push(AocFile { id: -1, size: size });
			}
			free_space = !free_space;
		}
	}
	files
}

fn p1_parse(file: File) -> Vec<i16> {
	let mut blocks = Vec::new();
	for line in io::BufReader::new(file).lines() {
		let l = line.unwrap();
		let mut free_space = false;
		let mut file_id = 0;
		for c in l.chars() {
			let size = c.to_digit(10).unwrap();
			for _ in 0..size {
				if !free_space {
					blocks.push(file_id);
				} else {
					blocks.push(-1);
				}
			}
			if !free_space {
				file_id += 1;
			}
			free_space = !free_space;
		}
	}
	blocks
}

fn main() {
	if false {
		let p1d = p1_parse(File::open("./input.txt").unwrap());
		println!("{}", p1(p1d));
	} else {
		let p2d = p2_parse(File::open("./input.txt").unwrap());
		println!("{}", p2(p2d));
	}
}

fn p1(mut blocks: Vec<i16>) -> u64 {
	for file_i in (0..blocks.len()).rev() {
		if blocks[file_i] == -1 {
			continue;
		}
		for free_i in 0..file_i {
			if blocks[free_i] != -1 {
				continue;
			}
			blocks[free_i] = blocks[file_i];
			blocks[file_i] = -1;
		}
	}

	let mut sum = 0;
	for i in 0..blocks.len() {
		if blocks[i] == -1 {
			break;
		}
		sum += i as u64 * blocks[i] as u64;
	}
	sum
}

fn p2(mut files: Vec<AocFile>) -> u64 {
	let mut visited = HashSet::new();
	'outer: loop {
		for file_i in (0..files.len()).rev() {
			if files[file_i].id == -1 || visited.contains(&files[file_i].id) {
				continue;
			}
			visited.insert(files[file_i].id);
			for free_i in 0..file_i {
				if files[free_i].id != -1 || files[free_i].size < files[file_i].size {
					continue;
				}
				let file = files[file_i];
				let mut free = files[free_i];
				let remaining = free.size - file.size;
				free.size = file.size;
				files[free_i] = file;
				files[file_i] = free;
				if remaining > 0 {
					// Make new free space file
					let new_file = AocFile {
						size: remaining,
						id: -1,
					};
					files.insert(free_i + 1, new_file);
					continue 'outer; // reset due to index changes
				} else {
					break; // go to next file
				}
			}
		}
		break;
	}

	let mut block_idx = 0;
	let mut sum = 0;
	for file in files {
		if file.id == -1 {
			block_idx += file.size as u64;
		} else {
			for _ in 0..file.size {
				sum += block_idx * file.id as u64;
				block_idx += 1;
			}
		}
	}
	sum
}
