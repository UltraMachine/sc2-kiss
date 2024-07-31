use sc2_core::{Client, Result};
use std::{env, net::SocketAddr};

fn main() {
	main2().unwrap_or_else(|e| eprintln!("{e}"))
}

fn main2() -> Result {
	let addr: SocketAddr = env::args()
		.nth(1)
		.unwrap_or_else(|| "[::1]:5000".into())
		.parse()
		.expect("Can't parse socket address");

	let url = format!("ws://{addr}/sc2api");
	let mut client = Client::connect(url)?;

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
