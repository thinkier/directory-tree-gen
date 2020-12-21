use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::excludes::Excludes;

pub fn generate_text<W: Write>(out: &mut W, path: &Path, excludes: &Excludes) -> Result<(), Box<dyn Error>> {
	writeln!(out, "{}/", to_name(path))?;

	generate_text_impl(out, path, excludes, "", 1)
}

fn to_name(path: &Path) -> String {
	path.file_name()
		.map(|x| format!("{}", x.to_string_lossy()))
		.unwrap_or_else(|| String::new())
}

fn generate_text_impl<W: Write>(out: &mut W, path: &Path, excludes: &Excludes, prefix: &str, depth: usize) -> Result<(), Box<dyn Error>> {
	if let Ok(dir) = fs::read_dir(path) {
		let mut paths = dir.filter(|x| x.is_ok())
			.map(|x| x.unwrap())
			.map(|x| x.path())
			.filter(|path| !excludes.contains(path, depth))
			.collect::<Vec<_>>();

		paths.sort_by(|a, b| a.cmp(b));
		paths.sort_by_key(|x| !x.is_dir());

		let len = paths.len();
		for i in 0..len {
			let is_dir = paths[i].is_dir();
			let last = i == len - 1;
			let item_prefix = if last {
				"└─"
			} else {
				"├─"
			};

			if let Some(name) = paths[i].file_name() {
				let name = name.to_string_lossy();
				write!(out, "{} {} {}", prefix, item_prefix, name)?;
				if is_dir {
					writeln!(out, "/")?;
					let item_prefix = if last {
						" "
					} else {
						"│"
					};

					generate_text_impl(out, &paths[i], excludes, &format!("{} {}  ", prefix, item_prefix), depth + 1)?;
				} else {
					writeln!(out)?;
				}
			}
		}
	}

	Ok(())
}
