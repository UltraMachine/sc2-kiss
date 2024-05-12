use std::net::IpAddr;

use sc2_core::{request::JoinCfg, Client, Result};

#[allow(clippy::needless_doctest_main)]
/**
Ladder CLI args

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
	ladder: Ladder,
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
	ladder: Ladder,
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
pub struct Ladder {
	/// IP of SC2 API server to connect with client
	#[cfg_attr(feature = "cli-bpaf", bpaf(long("LadderServer"), argument("IP")))]
	#[cfg_attr(feature = "cli-clap", arg(long("LadderServer"), value_name("IP")))]
	pub ip: IpAddr,
	/// Port of SC2 API server to connect with client
	#[cfg_attr(
		feature = "cli-bpaf",
		bpaf(long("GamePort"), argument("NUMBER"), fallback(5000))
	)]
	#[cfg_attr(
		feature = "cli-clap",
		arg(long("GamePort"), value_name("NUMBER"), default_value_t = 5000)
	)]
	pub port: u16,
	/// Port to use in `JoinGame` request
	#[cfg_attr(
		feature = "cli-bpaf",
		bpaf(long("StartPort"), argument("NUMBER"), fallback(5000))
	)]
	#[cfg_attr(
		feature = "cli-clap",
		arg(long("StartPort"), value_name("NUMBER"), default_value_t = 5000)
	)]
	pub join_port: u16,
	/// Opponent id on ladder
	#[cfg_attr(
		feature = "cli-bpaf",
		bpaf(long("OpponentId"), argument("STRING"), fallback(String::new()))
	)]
	#[cfg_attr(
		feature = "cli-clap",
		arg(long("OpponentId"), value_name("STRING"), default_value_t)
	)]
	pub opponent_id: String,
	/// Specifies if game will run in realtime more or step mode
	#[cfg_attr(feature = "cli-bpaf", bpaf(long("RealTime")))]
	#[cfg_attr(feature = "cli-clap", arg(long("RealTime")))]
	pub realtime: bool,
}
impl Ladder {
	/// Connects to the SC2 API with parsed IP and port
	pub fn connect(&self) -> Result<Client> {
		Client::connect(format!("ws://{}:{}/sc2api", self.ip, self.port))
	}
	/// Sets correct `server_ports` and `client_ports` in [`JoinCfg`]
	pub fn set_ports(&self, join_cfg: &mut JoinCfg) {
		let p = self.join_port;
		join_cfg.server_ports = Some((p + 1, p + 2).into());
		join_cfg.client_ports = vec![(p + 3, p + 4).into()];
	}
}
