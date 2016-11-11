extern crate babel;
extern crate clap;

use babel::{Address, read, search};
use clap::{Arg, App, SubCommand};
use std::process;

macro_rules! parse_address {
	($input:expr, $max:expr, $label:expr) => {
		match $input.parse::<u32>() {
			Ok(n) => {
				if n >= $max {
					println!("Bad address: {} must be less than {}", $label, $max);
					process::exit(1);
				}
				n
			},
			Err(_) => {
				println!("Bad address: Not a number.");
				process::exit(1);
			}
		}
	}
}

fn main() {

	let matches = App::new("The Library of Babel")
		.version(env!("CARGO_PKG_VERSION"))
		.about("Home of every book ever")
		.author("Brandon Aldrich <brandon@aldr.io>")
		.subcommand(SubCommand::with_name("search")
			.about("Search the library for something")
			.version(env!("CARGO_PKG_VERSION"))
			.arg(Arg::with_name("query")
				.required(true)
				.help("The search query")
				.use_delimiter(false))
			.arg(Arg::with_name("noisy")
				.long("noisy")
				.help("Allows pages with random characters around the query")))
		.subcommand(SubCommand::with_name("read")
			.about("Read a page from the library")
			.version(env!("CARGO_PKG_VERSION"))
			.arg(Arg::with_name("address")
				.required(true)
				.help("The address of the page to read from (wall:shelf:volume:page:hex_address)")))
		.get_matches();

	if let Some(matches) = matches.subcommand_matches("search") {
		
		let query = matches.value_of("query").unwrap();

		let query = if matches.is_present("noisy") {
			babel::pad_rand(query)
		} else {
			query.to_owned()
		};

		let addr = search(&query);

		println!("Page found!\n\"{}\"", addr);

	} else if let Some(matches) = matches.subcommand_matches("read") {
		let address = matches.value_of("address").unwrap();

		let split: Vec<&str> = address.split(':').collect();

		if split.len() != 5 {
			println!("Bad address: (wall:shelf:volume:page:hex_address)");
			process::exit(1);
		}

		let addr = Address {
			hex: split[4].to_owned(),
			wall: parse_address!(split[0], 4, "Wall"),
			shelf: parse_address!(split[1], 5, "Shelf"),
			volume: parse_address!(split[2], 32, "Volume"),
			page: parse_address!(split[3], 410, "Page"),
		};

		let page = read(&addr);
		print_formatted(&page);
	}
}

/// Prints the page with correct formatting
fn print_formatted(page: &str) {
	for (i, a) in page.chars().enumerate() {
		print!("{}", a);
		if (i + 1) % 80 == 0 {
			println!("");
		}
	}
}
