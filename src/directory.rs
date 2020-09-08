use std::fmt::{self, Formatter, Result as FmtResult};
use std::fs;
use std::path::Path;

use crate::Excludes;

#[derive(Debug)]
pub enum DirectoryTree {
	File(String),
	Folder(String, Vec<DirectoryTree>),
}

impl DirectoryTree {
	pub fn name(&self) -> &str {
		match self {
			DirectoryTree::File(x) => &x,
			DirectoryTree::Folder(x, _) => &x
		}
	}

	pub fn is_dir(&self) -> bool {
		if let DirectoryTree::File(_) = self {
			return false;
		}

		return true;
	}

	pub fn format(&self) -> String {
		match self {
			DirectoryTree::File(name) => {
				format!("{}\n", name)
			}
			DirectoryTree::Folder(name, children) => {
				let mut buf = format!("{}/\n", name);

				let count = children.len();
				for i in 0..count {
					let last = i == count - 1;

					let mut related = true;
					for line in format!("{}", children[i]).lines() {
						let prefix = if related {
							if last {
								"└─"
							} else {
								"├─"
							}
						} else {
							if last {
								"  "
							} else {
								"│ "
							}
						};

						buf += &format!(" {} {}\n", prefix, line);
						related = false;
					}
				}

				buf
			}
		}
	}

	pub fn from_dir(path: &Path, excludes: &Excludes) -> Self {
		DirectoryTree::Folder(path.file_name().unwrap().to_string_lossy().to_string(), Self::recurse(path, &excludes, 1))
	}

	fn recurse(path: &Path, excludes: &Excludes, depth: usize) -> Vec<Self> {
		let dir = if let Ok(x) = fs::read_dir(path) {
			x
		} else {
			return vec![DirectoryTree::File(path.file_name().unwrap().to_string_lossy().to_string())];
		};

		let mut children: Vec<_> = dir.into_iter()
			.filter(|item| item.is_ok())
			.map(|item| item.unwrap())
			.map(|item| item)
			.filter_map(|item| {
				let file_name = item.file_name().to_string_lossy().to_string();
				let path = item.path();

				if excludes.contains(&path, depth) {
					return None;
				}

				let folder = fs::read_dir(&path).is_ok();
				Some(if folder {
					DirectoryTree::Folder(file_name, Self::recurse(&path, &excludes, depth + 1))
				} else {
					DirectoryTree::File(file_name)
				})
			})
			.collect();

		children.sort_unstable_by(|a, b| {
			a.name().cmp(b.name())
		});
		children.sort_by(|a, b| {
			b.is_dir().cmp(&a.is_dir())
		});

		return children;
	}
}

impl fmt::Display for DirectoryTree {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.format())
	}
}
