#![allow(clippy::tabs_in_doc_comments)]

/*!
Core crate of `sc2-kiss` framework.
Provides basic functionality to connect and communicate with SC2 instances.

Start by looking into [`Client`] documentation.

## Todo
- [x] Client connecting and request sending
- [x] Check response kind/status/errors
- [ ] Optional methods for easier request making
- [ ] SC2 Instance launching
- [ ] Proxy between client and sc2
*/

use thiserror::Error;
use tungstenite::stream::MaybeTlsStream;

pub mod common;
#[cfg(feature = "request-methods")]
pub mod request;

use common::{internal::*, *};

pub use sc2_prost::{request::Request as Req, response::Response as ResVar, Status};
pub use tungstenite::client::IntoClientRequest as ToUrl;

/// Possible [`Client`] errors
#[derive(Debug, Error)]
pub enum Error {
	/// WebSocket failure
	#[error("WebSocket Error: {0}")]
	WebSocket(#[from] tungstenite::Error),
	/// Error decoding response
	#[error("Decode Error: {0}")]
	Decode(#[from] prost::DecodeError),
	/// Response [`Kind`] doesn't match request [`Kind`]
	#[error("Bad Response: `{0:?}`, expected `{1:?}`")]
	BadRes(Kind, Kind),
	/// The server [`Status`] didn't change to one of the expected states after the request
	#[error("Bad Status: `{0:?}`, expected any of {1:?}")]
	BadStatus(Status, Vec<Status>),
	/// Response contains some errors
	#[error("{0}")]
	Sc2(#[from] Sc2Error),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

type WebSocket = tungstenite::WebSocket<MaybeTlsStream<std::net::TcpStream>>;

/**
Client interface to connect and communicate with SC2 instance.

Basic usage:
```
use sc2_core::{Client, Req};

let mut client = Client::connect("ws://localhost:5000/sc2api")?;
let res = client.send(Req::Ping(Default::default()))?;
println!("{res:?}");
# Ok::<(), sc2_core::Error>(())
```
*/
#[derive(Debug)]
pub struct Client {
	ws: WebSocket,
	status: Status,
}
impl Client {
	/**
	Connects to the given SC2API WebSocket server and returns [`Client`].

	# Errors

	This function can error in case of invalid URL or connection failure.

	# Examples
	```
	use sc2_core::Client;

	let mut client = Client::connect("ws://127.0.0.1:5000/sc2api")?;
	# Ok::<(), sc2_core::Error>(())
	```
	```
	use sc2_core::Client;
	use url::Url;

	let url = Url::parse("ws://127.0.0.1:5000/sc2api").unwrap();
	let mut client = Client::connect(url)?;
	# Ok::<(), sc2_core::Error>(())
	```
	```
	use sc2_core::Client;
	use http::Uri;

	let uri = "ws://127.0.0.1:5000/sc2api".parse::<Uri>().unwrap();
	let mut client = Client::connect(uri)?;
	# Ok::<(), sc2_core::Error>(())
	```
	*/
	pub fn connect(to: impl ToUrl) -> Result<Self> {
		let (ws, _) = tungstenite::connect(to)?;
		Ok(Self {
			ws,
			status: Status::Unset,
		})
	}

	/**
	Sends given request and returns received response.

	# Errors

	This method errors in the following cases:
	- WebSocket failure
	- Failed to decode response
	- Response [`Kind`] doesn't match request [`Kind`]
	- The server [`Status`] didn't change to one of the expected states after the request
	- Response contains any errors

	# Examples
	```
	# let mut client = sc2_core::Client::connect("ws://localhost:5000/sc2api")?;
	use sc2_core::{Req, ResVar};

	let res = client.send(Req::Ping(Default::default()))?;
	println!("Server Status: {:?}", res.status);
	let ResVar::Ping(data) = res.data else { unreachable!() };
	println!("Game Version: {}", data.game_version);
	println!("Data Version: {}", data.data_version);
	println!("Data Build: {}", data.data_build);
	println!("Base Build: {}", data.base_build);
	# Ok::<(), sc2_core::Error>(())
	```
	```
	# let mut client = sc2_core::Client::connect("ws://localhost:5000/sc2api")?;
	use sc2_core::{Req, ResVar};

	let res = client.send(Req::AvailableMaps(Default::default()))?;
	println!("Server Status: {:?}", res.status);
	let ResVar::AvailableMaps(data) = res.data else { unreachable!() };

	println!("Local maps:");
	for map in data.local_map_paths {
		println!("- {map}");
	}
	println!("BattleNet maps:");
	for map in data.battlenet_map_names {
		println!("- {map}");
	}
	# Ok::<(), sc2_core::Error>(())
	```
	```no_run
	# let mut client = sc2_core::Client::connect("ws://localhost:5000/sc2api")?;
	use sc2_core::{Req, ResVar};

	let req = sc2_prost::RequestJoinGame {
		// Set your options here
		..Default::default()
	};
	let res = client.send(Req::JoinGame(req))?;
	println!("Server Status: {:?}", res.status);
	let ResVar::JoinGame(data) = res.data else { unreachable!() };
	println!("Our Player Id: {}", data.player_id);
	# Ok::<(), sc2_core::Error>(())
	```
	*/
	pub fn send(&mut self, req: Req) -> Result<Res> {
		let req_kind = req.kind();
		self.ws.send(req_into_msg(req))?;
		let res = res_from_msg(self.ws.read()?, req_kind)?;
		check_res(res, req_kind, &mut self.status)
	}

	/**
	Returns current server [`Status`].

	# Examples
	```
	# let mut client = sc2_core::Client::connect("ws://localhost:5000/sc2api")?;
	use sc2_core::{Req, Status};

	assert_eq!(client.status(), Status::Unset);

	let res = client.send(Req::Ping(Default::default()))?;
	println!("Server Status: {:?}", res.status);

	assert_eq!(client.status(), Status::Launched);
	assert_eq!(client.status(), res.status);
	# Ok::<(), sc2_core::Error>(())
	```
	*/
	pub fn status(&self) -> Status {
		self.status
	}
}
