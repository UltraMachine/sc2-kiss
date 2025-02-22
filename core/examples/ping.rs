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

	let res = client.ping()?;
	println!("Status: {:?}", res.status);
	if !res.warns.is_empty() {
		println!("Warnings:");
		for (i, msg) in (1..).zip(res.warns) {
			println!("{i}. {msg}");
		}
	}
	println!();
	println!("Game Version: {}", res.data.game_version);
	println!("Data Version: {}", res.data.data_version);
	println!("Data Build: {}", res.data.data_build);
	println!("Base Build: {}", res.data.base_build);

	Ok(())
}
