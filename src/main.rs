use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;

fn main() {
	// path to file
	let path = Path::new("lib/commoncap.txt");

	// open it in read-only mode
	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open: {}", why.description()),
		Ok(file) => file,
	};

	// write to string
	let mut file_content = String::new();
	match file.read_to_string(&mut file_content) {
		Err(why) => panic!("couldn't read: {}", why.description()),
		Ok(_) => (),
	}

	println!("file: {}", file_content);
}