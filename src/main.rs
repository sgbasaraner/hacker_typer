extern crate pancurses;

use pancurses::{initscr, noecho, has_colors, start_color};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;

fn main() {
	// path to file
	let path = Path::new("lib/commoncap.txt");

	// open the file in read-only mode
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
	let mut counter = 0; // to keep track of where we are in the file

	// initialize curses window
	let window = initscr();
	window.refresh();
	window.keypad(true);
	noecho();

	// scrolling
	window.setscrreg(10, 10);
	window.scrollok(true);

	loop {
	    match window.getch() {
	        Some(_) => { 
				if counter < file_length {
	        		let mut tmp = 0;
	        		while tmp < 3 && counter < file_length {
						// write three characters for each key press
	        			window.addch(file_content.chars().nth(counter).unwrap());
	        			tmp += 1;
	        			counter += 1;
	        		}
	        	} else {
					// restart if we reached the end of the file
	        		counter = 0;
	        	}},
	        None => ()
	    }
	}
}