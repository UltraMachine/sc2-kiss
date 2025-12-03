#[cfg(feature = "ai-arena")]
use sc2_core::request::setup::{JoinGame, join_game};
use sc2_core::{Client, Result};
use std::net::IpAddr;

#[cfg(feature = "ai-arena")]
#[allow(clippy::needless_doctest_main)]
/**
AI Arena Ladder CLI args

# Examples
<details>
<summary>Usage with <code>bpaf</code></summary>

```no_run
use bpaf::Bpaf;
use sc2_kiss::cli::*;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Cli {
	#[bpaf(external)]
	ladder: AiArena,
}

fn main() {
	println!("{:?}", cli().run());
}
```
</details>

<details>
<summary>Usage with <code>clap</code></summary>

```no_run
use clap::Parser;
use sc2_kiss::cli::*;

#[derive(Debug, Clone, Parser)]
struct Cli {
	#[command(flatten)]
	ladder: AiArena,
}

fn main() {
	println!("{:?}", Cli::parse());
}
```
</details>
*/
#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli-bpaf", derive(bpaf::Bpaf))]
#[cfg_attr(feature = "cli-clap", derive(clap::Args))]
pub struct AiArena {
	#[cfg_attr(feature = "cli-bpaf", bpaf(long("LadderServer"), argument("IP")))]
	#[cfg_attr(feature = "cli-clap", arg(long("LadderServer"), value_name("IP")))]
	/// IP of SC2 API server to connect with client
	pub ip: IpAddr,

	#[cfg_attr(
		feature = "cli-bpaf",
		bpaf(long("GamePort"), argument("NUMBER"), fallback(5000))
	)]
	#[cfg_attr(
		feature = "cli-clap",
		arg(long("GamePort"), value_name("NUMBER"), default_value_t = 5000)
	)]
	/// Port of SC2 API server to connect with client
	pub port: u16,

	#[cfg_attr(
		feature = "cli-bpaf",
		bpaf(long("StartPort"), argument("NUMBER"), fallback(5000))
	)]
	#[cfg_attr(
		feature = "cli-clap",
		arg(long("StartPort"), value_name("NUMBER"), default_value_t = 5000)
	)]
	/// Port to use in `JoinGame` request
	pub join_port: u16,

	#[cfg_attr(
		feature = "cli-bpaf",
		bpaf(long("OpponentId"), argument("STRING"), fallback(String::new()))
	)]
	#[cfg_attr(
		feature = "cli-clap",
		arg(long("OpponentId"), value_name("STRING"), default_value_t)
	)]
	/// Opponent id on ladder
	pub opponent_id: String,

	#[cfg_attr(feature = "cli-bpaf", bpaf(long("RealTime")))]
	#[cfg_attr(feature = "cli-clap", arg(long("RealTime")))]
	/// Specifies if game will run in realtime more or step mode
	pub realtime: bool,
}
#[cfg(feature = "ai-arena")]
impl AiArena {
	/// Connects to the SC2 API with parsed IP and port
	pub fn connect(&self) -> Result<Client> {
		Client::connect((self.ip, self.port))
	}
	pub fn join_game(&self) -> JoinGame {
		let p = self.join_port;
		join_game()
			.server_ports((p + 1, p + 2))
			.client_ports(vec![(p + 3, p + 4).into()])
	}
}
