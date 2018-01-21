extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho};
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

	let file_length = file_content.len();
	let mut counter = 0;

	let window = initscr();
	window.refresh();
	window.keypad(true);
	noecho();
	loop {
	    match window.getch() {
	        Some(Input::Character(_)) => {
	        	if counter < file_length {
	        		let mut tmp = 0;
	        		while tmp < 3 && counter < file_length {
	        			window.addch(file_content.chars().nth(counter).unwrap());
	        			tmp += 1;
	        			counter += 1;
	        		}
	        	} else {
	        		counter = 0;
	        	}},
	        Some(Input::KeyDC) => break,
	        Some(input) => { window.addstr(&format!("{:?}", input)); },
	        None => ()
	    }
	}
	endwin();
}