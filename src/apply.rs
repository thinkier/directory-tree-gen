use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::mpsc::Receiver;

const HEADING: &str = "# project directory structure";

pub fn apply_dir_tree(path: &Path, dir: Receiver<Vec<u8>>) -> Result<(), Box<dyn Error>> {
	let (start, _old_tree, end) = get_existing_content(path)?;

	let mut file = fs::OpenOptions::new()
		.create(true)
		.write(true)
		.truncate(true)
		.open(path)?;

	file.write(start.as_bytes())?;

	while let Ok(line) = dir.recv() {
		file.write(&line)?;
	}

	file.write(end.as_bytes())?;

	Ok(())
}

fn get_existing_content(path: &Path) -> Result<(String, String, String), Box<dyn Error>> {
	let file = fs::OpenOptions::new()
		.create(true)
		.write(true)
		.read(true)
		.open(path)?;

	let mut read = BufReader::new(file);

	let mut start = String::new();
	let mut tree = String::new();

	let mut skip = false;
	let mut heading = false;

	let mut line = String::new();
	while let Ok(n) = read.read_line(&mut line) {
		if n == 0 { break; }

		let trimmed = line.trim_end_matches(|ch| ch == '\r' || ch == '\n');

		if heading {
			if skip {
				let end_of_block = trimmed == "```";

				if !end_of_block {
					tree += trimmed;
					tree += "\n";
				}

				line = String::new();
				if end_of_block {
					break;
				}
				continue;
			}

			skip = trimmed.starts_with("```");
		}


		heading = heading || trimmed.to_lowercase().ends_with(HEADING);
		start += trimmed;
		start += "\n";
		line = String::new();
	}

	if !heading {
		start += HEADING;
		start += "\n";
	}

	if !skip {
		start += "```\n";
	}

	let mut end = "```\n".to_string();

	let mut line = String::new();
	while let Ok(n) = read.read_line(&mut line) {
		if n == 0 { break; }
		end += line.trim_end_matches(|ch| ch == '\r' || ch == '\n');
		end += "\n";

		line = String::new();
	}

	return Ok((
		start,
		tree,
		end
	));
}