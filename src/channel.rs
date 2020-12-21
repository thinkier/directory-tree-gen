use std::{io, mem};
use std::io::Write;
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};

pub struct Channel<T> {
	buffer: Vec<u8>,
	sender: SyncSender<T>,
}

impl<T: From<Vec<u8>>> Channel<T> {
	pub fn create() -> (Self, Receiver<T>) {
		let (sender, rx) = sync_channel(32);

		(Channel {
			buffer: Vec::new(),
			sender,
		}, rx)
	}

	fn next_line(&self) -> Option<usize> {
		for i in 0..self.buffer.len() {
			if self.buffer[i] == b'\n' {
				return Some(i);
			}
		}

		return None;
	}

	fn soft_flush(&mut self) {
		while let Some(ln) = self.next_line() {
			let mut line = self.buffer.split_off(ln + 1);

			mem::swap(&mut line, &mut self.buffer);

			self.sender.send(line.into()).unwrap();
		}
	}
}

impl<T: From<Vec<u8>>> Write for Channel<T> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.buffer.extend_from_slice(buf);

		self.soft_flush();
		Ok(buf.len())
	}

	fn flush(&mut self) -> io::Result<()> {
		self.soft_flush();
		let buf = mem::replace(&mut self.buffer, Vec::new());

		self.sender.send(buf.into()).unwrap();

		Ok(())
	}
}