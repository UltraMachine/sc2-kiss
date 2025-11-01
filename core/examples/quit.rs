use sc2_core::{Client, Result, request::Quit};
use std::env;

fn main() {
	run().unwrap_or_else(|e| eprintln!("{e}"))
}

fn run() -> Result {
	let addr = env::args()
		.nth(1)
		.unwrap_or_else(|| "localhost:5000".into());

	let mut client = Client::connect(addr)?;

	let res = client.request(Quit)?;
	println!("Status: {:?}", res.status);
	if !res.warnings.is_empty() {
		println!("Warnings:");
		for (i, msg) in (1..).zip(res.warnings) {
			println!("{i}. {msg}");
		}
	}

	Ok(())
}
