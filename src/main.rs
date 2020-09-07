use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::directory::DirectoryTree;

#[derive(Clone, Debug)]
pub struct Excludes {
	relative: Vec<String>,
	absolute: Vec<String>,
}

macro_rules! string_vec (($($item:expr),*) => (
	vec![
		$($item.to_string()),*
	]
));

impl Default for Excludes {
	fn default() -> Self {
		Self {
			absolute: vec![],
			relative: string_vec![".git", ".idea", "*.iml", "target", "node_modules"],
		}
	}
}

impl Excludes {
	pub fn contains(&self, path: &Path, take_frags: usize) -> bool {
		let frags: Vec<_> = path.iter()
			.collect();

		let item: String = frags.iter()
			.skip(frags.len() - take_frags)
			.map(|x| x.to_string_lossy())
			.collect::<Vec<_>>()
			.join("/");

		if self.relative.iter()
			.any(|needle| {
				if needle.starts_with("*") && item.ends_with(&needle[1..]) {
					return true;
				}

				item.contains(needle)
			}) {
			return true;
		}

		self.absolute.iter()
			.any(|needle| item.starts_with(needle))
	}

	pub fn from_ignores() -> Self {
		let ignores = if let Ok(file) = fs::OpenOptions::new()
			.read(true)
			.write(false)
			.open(".gitignore") {
			let buf = BufReader::new(file);

			buf.lines()
				.filter(|x| x.is_ok())
				.map(|x| x.unwrap())
				.filter(|x| x.len() > 0)
				.filter(|x| !x.starts_with("#"))
				.map(|x| x.trim_end_matches("/").to_string())
		} else {
			return Default::default();
		};

		let mut proto = Self::default();

		for ignore in ignores {
			if ignore.starts_with("/") {
				proto.absolute.push(ignore.trim_start_matches("/").to_string());
			} else {
				proto.relative.push(ignore);
			}
		}

		return proto;
	}
}

mod directory;

fn main() {
	let excludes = Excludes::from_ignores();

	println!("{}", DirectoryTree::from_dir(&env::current_dir().unwrap(), &excludes));
}
