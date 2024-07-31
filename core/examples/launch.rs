use sc2_core::instance::Launcher;
use std::io;

fn main() {
	main2().unwrap_or_else(|e| eprintln!("{e}"))
}

fn main2() -> io::Result<()> {
	let launcher = Launcher {
		game_dir: "C:/games/StarCraft II".into(),
		..Default::default()
	};
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
