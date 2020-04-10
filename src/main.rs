use std::env;

mod lib;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		panic!("not enough args");
	}

	println!("0x{:08X}", lib::djb2(&args[1]));
}