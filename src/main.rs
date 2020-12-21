use std::env;
use std::error::Error;
use std::io::stdout;

use crate::directory::generate_text;
use crate::excludes::Excludes;

mod directory;
mod excludes;

fn main() -> Result<(), Box<dyn Error>> {
	let excludes = Excludes::from_ignores();

	generate_text(&mut stdout(), &env::current_dir().unwrap(), &excludes)
}
