use sc2_core::{Client, Result};
use std::env;

fn main() {
	run().unwrap_or_else(|e| eprintln!("{e}"))
}

fn run() -> Result {
	let addr = env::args()
		.nth(1)
		.map_or_else(|| "[::1]:5000".parse(), |s| s.parse())
		.expect("Can't parse socket address");

	let mut client = Client::connect_addr(addr)?;

	let data = client.available_maps()?.data;
	println!("Local maps:");
	for map in data.local_map_paths {
		println!("- {map}");
	}
	println!("BattleNet maps:");
	for map in data.battlenet_map_names {
		println!("- {map}");
	}

	Ok(())
}
