use sc2_core::{Client, Result};
use std::env;

fn main() {
	run().unwrap_or_else(|e| eprintln!("{e}"))
}

fn run() -> Result {
	let addr = env::args()
		.nth(1)
		.unwrap_or_else(|| "localhost:5000".into());

	let mut client = Client::connect(addr)?;

	let res = client.quit()?;
	println!("Status: {:?}", res.status);
	if !res.warns.is_empty() {
		println!("Warnings:");
		for (i, msg) in (1..).zip(res.warns) {
			println!("{i}. {msg}");
		}
	}

	Ok(())
}
