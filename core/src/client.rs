use super::*;
use std::{
	io,
	net::{SocketAddr, ToSocketAddrs},
	thread::sleep,
	time::{Duration, Instant},
};
use tungstenite::{
	Error as WsError, client::IntoClientRequest, error::UrlError, http, stream::MaybeTlsStream,
};

type WebSocket = tungstenite::WebSocket<MaybeTlsStream<std::net::TcpStream>>;

/// Possible [`Client`] errors
#[derive(Debug, Error)]
pub enum Error {
	/// WebSocket failure
	#[error("WebSocket error: {0}")]
	WebSocket(#[from] WsError),
	/// Error decoding response
	#[error("Decode error: {0}")]
	Decode(#[from] prost::DecodeError),
	/// Response [`Kind`] doesn't match request [`Kind`]
	#[error("Bad response: `{0:?}`, expected `{1:?}`")]
	BadRes(Kind, Kind),
	/// The server [`Status`] didn't change to one of the expected states after the request
	#[error("Bad status: `{0:?}`, expected any of {1:?}")]
	BadStatus(Status, Vec<Status>),
	/// Response contains some errors
	#[error("{0}")]
	Sc2(#[from] Sc2Error),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/**
Client interface to connect and communicate with SC2 instance.

Basic usage:
```no_run
use sc2_core::{Client, Req};

let mut client = Client::connect("localhost:5000")?;
let res = client.request(Req::Ping(Default::default()))?;
println!("{res:?}");
```
*/
#[derive(Debug)]
pub struct Client {
	ws: WebSocket,
	status: Status,
}
/// Core methods
impl Client {
	fn connect_imp(http_req: http::Request<()>) -> Result<Self> {
		let (ws, _) = tungstenite::connect(http_req)?;
		Ok(Self {
			ws,
			status: Status::Unset,
		})
	}

	/**
	Connects to the given SC2 API WebSocket server and returns [`Client`].

	# Errors

	This function can error in case of invalid address or connection failure.

	# Examples
	```no_run
	use sc2_core::Client;

	let mut client = Client::connect("localhost:5000")?;
	```
	*/
	pub fn connect(addr: impl ToSocketAddrs) -> Result<Self> {
		let addrs = addr.to_socket_addrs().map_err(WsError::Io)?;
		let mut last_err = None;
		for addr in addrs {
			let http_req = format!("ws://{addr}/sc2api").into_client_request()?;
			match Self::connect_imp(http_req) {
				ok @ Ok(_) => return ok,
				Err(e) => last_err = Some(e),
			}
		}
		Err(last_err.unwrap_or_else(|| {
			Error::WebSocket(WsError::Io(io::Error::new(
				io::ErrorKind::InvalidInput,
				"could not resolve to any addresses",
			)))
		}))
	}

	pub fn connect_timeout(addr: SocketAddr, timeout: Duration) -> Result<Self> {
		let http_req = format!("ws://{addr}/sc2api").into_client_request()?;
		// todo: better way to await the connection?
		let now = Instant::now();
		loop {
			match Self::connect_imp(http_req.clone()) {
				ok @ Ok(_) => return ok,
				Err(Error::WebSocket(WsError::Url(UrlError::UnableToConnect(_)))) => {}
				err @ Err(_) => return err,
			}
			if now.elapsed() >= timeout {
				let e = io::Error::new(io::ErrorKind::TimedOut, "connection timed out");
				return Err(Error::WebSocket(WsError::Io(e)));
			}
			sleep(Duration::from_secs(1));
		}
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
	```no_run
	use sc2_core::{Req, ResVar};

	let res = client.request(Req::Ping(Default::default()))?;
	println!("Server Status: {:?}", res.status);
	let ResVar::Ping(data) = res.data else { unreachable!() };
	println!("Game Version: {}", data.game_version);
	println!("Data Version: {}", data.data_version);
	println!("Data Build: {}", data.data_build);
	println!("Base Build: {}", data.base_build);
	```
	```no_run
	use sc2_core::{Req, ResVar};

	let res = client.request(Req::AvailableMaps(Default::default()))?;
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
	```
	```no_run
	use sc2_core::{Req, ResVar};

	let req = sc2_prost::RequestJoinGame {
		// Set your options here
		..Default::default()
	};
	let res = client.request(Req::JoinGame(req))?;
	println!("Server Status: {:?}", res.status);
	let ResVar::JoinGame(data) = res.data else { unreachable!() };
	println!("Our Player Id: {}", data.player_id);
	```
	*/
	pub fn request(&mut self, req: Req) -> Result<Res> {
		let req_kind = req.kind();
		self.send(req)?;
		self.read(req_kind)
	}

	pub fn send(&mut self, req: Req) -> Result {
		self.ws.send(req_into_msg(req)).map_err(Into::into)
	}
	pub fn read(&mut self, expect_kind: Kind) -> Result<Res> {
		let res = res_from_msg(self.ws.read()?, expect_kind)?;
		check_res(res, expect_kind, &mut self.status)
	}

	/**
	Returns current server [`Status`].

	# Examples
	```no_run
	use sc2_core::{Req, Status};

	assert_eq!(client.status(), Status::Unset);

	let res = client.request(Req::Ping(Default::default()))?;
	println!("Server Status: {:?}", res.status);

	assert_eq!(client.status(), Status::Launched);
	assert_eq!(client.status(), res.status);
	```
	*/
	pub fn status(&self) -> Status {
		self.status
	}
}
