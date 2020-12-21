use std::env;

use crate::directory::DirectoryTree;
use crate::excludes::Excludes;

mod directory;
mod excludes;

fn main() {
	let excludes = Excludes::from_ignores();

	println!("{}", DirectoryTree::from_dir(&env::current_dir().unwrap(), &excludes));
}
