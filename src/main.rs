extern crate argh;

use std::{env, process};
use std::error::Error;
use std::io::stdout;
use std::path::PathBuf;

use argh::FromArgs;

use crate::directory::generate_text;
use crate::excludes::Excludes;

mod directory;
mod excludes;

#[derive(FromArgs)]
/// Directory tree generator
struct MainArgs {
	/// the directory to generate a tree for, defaults to current directory.
	#[argh(option)]
	dir: Option<String>
}

fn main() -> Result<(), Box<dyn Error>> {
	let args: MainArgs = argh::from_env();

	let path = args.dir
		.map(|dir| {
			match PathBuf::from(&dir).canonicalize() {
				Ok(x) => x,
				Err(_) => {
					eprintln!("File/folder not found!");
					process::exit(1);
				}
			}
		})
		.unwrap_or_else(|| env::current_dir().unwrap());

	let excludes = Excludes::from_ignores();

	generate_text(&mut stdout(), &path, &excludes)
}
