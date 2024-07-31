use sc2_core::{Client, Result};

fn main() {
	main2().unwrap_or_else(|e| eprintln!("{e}"))
}

fn main2() -> Result {
	let mut client = Client::connect("ws://[::1]:5000/sc2api")?;
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
