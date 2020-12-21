#![feature(iterator_fold_self)]

extern crate argh;

use std::{env, process, thread};
use std::error::Error;
use std::io::stdout;
use std::path::PathBuf;

use argh::FromArgs;

use crate::apply::apply_dir_tree;
use crate::channel::Channel;
use crate::directory::generate_text;
use crate::excludes::Excludes;

mod apply;
mod channel;
mod directory;
mod excludes;

#[derive(FromArgs)]
/// Directory tree generator
struct MainArgs {
	/// write the directory tree to the README.md file at the current (or specified) directory
	#[argh(switch, short = 'a')]
	apply: bool,

	/// the directory to generate a tree for, defaults to current directory.
	#[argh(option)]
	dir: Option<String>,

	/// the file that describes what the ignore, defaults to .gitignore
	#[argh(option)]
	ignore_file: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let args: MainArgs = argh::from_env();

	let mut path = args.dir
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

	let excludes = Excludes::from_file(if let Some(ignore) = &args.ignore_file {
		ignore
	} else {
		".gitignore"
	});

	if !args.apply {
		return generate_text(&mut stdout(), &path, &excludes);
	}

	let (mut tx, rx) = Channel::create();
	{
		let path = path.clone();
		thread::spawn(move || {
			generate_text(&mut tx, &path, &excludes).unwrap();
		});
	}

	path.push("README.md");
	apply_dir_tree(&path, rx)?;

	Ok(())
}
