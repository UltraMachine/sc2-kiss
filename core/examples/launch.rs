use sc2_core::instance::{DisplayMode::Windowed, Launcher, OnDrop::Kill, Result};
use std::env;

fn main() {
	run().unwrap_or_else(|e| eprintln!("{e}"))
}

fn run() -> Result<()> {
	let addr = env::args()
		.nth(1)
		.map_or_else(|| "[::1]:5000".parse(), |s| s.parse())
		.expect("Can't parse socket address");

	let launcher = Launcher {
		addr,
		on_drop: Kill,
		display_mode: Windowed,
		..Default::default()
	};
	println!("Command: {:?}", launcher.command()?);

	let mut instance = launcher.spawn()?;
	println!("Spawned instance");

	match instance.child.wait()?.code() {
		Some(code) => println!("Exited with code: {code}"),
		None => println!("Terminated by signal"),
	}

	Ok(())
}
