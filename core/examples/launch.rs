use sc2_core::instance::{Launcher, Result};
use std::env;

fn main() {
	run().unwrap_or_else(|e| eprintln!("{e}"))
}

fn run() -> Result<()> {
	let mut args = env::args();
	let game_dir = args
		.nth(1)
		.map_or_else(|| "/games/StarCraft II".into(), Into::into);
	let addr = args
		.next()
		.map_or_else(|| "[::1]:5000".parse(), |s| s.parse())
		.expect("Can't parse socket address");

	let launcher = Launcher::with_addr(addr, game_dir);
	println!("Command: {:?}", launcher.command()?);
	let mut instance = launcher.spawn()?;
	println!("Spawned instance");
	instance.kill_on_drop();
	match instance.child.wait()?.code() {
		Some(code) => println!("Exited with code: {code}"),
		None => println!("Terminated by signal"),
	}

	Ok(())
}
