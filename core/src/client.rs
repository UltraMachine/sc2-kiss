use super::*;
use std::{
	io,
	net::{SocketAddr, TcpStream, ToSocketAddrs},
	time::Duration,
};
use tungstenite::{ClientHandshake, Error as WsError, HandshakeError};

type WebSocket = tungstenite::WebSocket<TcpStream>;
type HsError = HandshakeError<ClientHandshake<TcpStream>>;

/// Possible [`Client`] errors
#[derive(Debug, Error)]
pub enum Error {
	#[error("Handshake error: {0}")]
	Handshake(Box<HsError>),
	/// WebSocket failure
	#[error("WebSocket error: {0}")]
	WebSocket(#[from] WsError),
	/// Error decoding response
	#[error("Decode error: {0}")]
	Decode(#[from] prost::DecodeError),
	/// Response [`Kind`] doesn't match request [`Kind`]
	#[error("{0}")]
	BadRes(#[from] BadResError),
	/// Response contains some errors
	#[error("{0}")]
	Sc2(#[from] Sc2Error),
}
impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		WsError::Io(e).into()
	}
}
impl From<HsError> for Error {
	fn from(e: HsError) -> Self {
		Self::Handshake(Box::new(e))
	}
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/**
Client interface to connect and communicate with SC2 instance.

Basic usage:
```no_run
use sc2_core::{Client, request::Ping};

let mut client = Client::connect("localhost:5000")?;
let res = client.request(Ping)?;
println!("{res:?}");
```
*/
#[derive(Debug)]
pub struct Client {
	ws: WebSocket,
}
/// Core methods
impl Client {
	fn _connect(stream: TcpStream) -> Result<Self> {
		stream.set_nodelay(true)?;
		let req = format!("ws://{}/sc2api", stream.peer_addr()?);
		let (ws, _) = tungstenite::client(req, stream)?;
		Ok(Self { ws })
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
		Self::_connect(TcpStream::connect(addr)?)
	}

	pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> Result<Self> {
		Self::_connect(TcpStream::connect_timeout(addr, timeout)?)
	}

	/**
	Sends given request and returns received response.

	# Errors

	This method errors in the following cases:
	- WebSocket failure
	- Failed to decode response
	- Response [`Kind`] doesn't match request [`Kind`]
	- Response contains any errors

	# Examples
	```no_run
	use sc2_core::request::Ping;

	let res = client.request(Ping)?;
	println!("Server Status: {:?}", res.status);
	let data = res.data;
	println!("Game Version: {}", data.game_version);
	println!("Data Version: {}", data.data_version);
	println!("Data Build: {}", data.data_build);
	println!("Base Build: {}", data.base_build);
	```
	```no_run
	use sc2_core::request::AvailableMaps;

	let res = client.request(AvailableMaps)?;
	println!("Server Status: {:?}", res.status);

	let data = res.data;
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
	use sc2_core::{
		request::{interface, join_game},
		sc2_prost::Race,
	};

	let req = join_game()
		.participant(Race::Random)
		.name("Bot".to_string())
		.interface(
			interface()
				.raw(true)
				.cloaked(true)
				.burrowed(true)
				.crop_raw(true),
		);
	let res = client.request(req)?;
	println!("Server Status: {:?}", res.status);
	println!("Our Player Id: {}", res.data.player_id);
	```
	*/
	pub fn request<R: ToRequest>(&mut self, request: R) -> Result<Res<R::Data>> {
		fn _request(client: &mut Client, request: Request, kind: Kind) -> Result<Res> {
			client.send(request)?;
			let response = client.read()?;
			convert_res(response, kind)
		}
		let kind = request.kind();
		_request(self, request.into(), kind).try_map_res(R::map_res)
	}

	pub fn read(&mut self) -> Result<Response> {
		res_from_msg(self.ws.read()?)
	}
	pub fn send(&mut self, request: Request) -> Result {
		Ok(self.ws.send(req_into_msg(request))?)
	}

	pub fn write(&mut self, request: Request) -> Result {
		Ok(self.ws.write(req_into_msg(request))?)
	}
	pub fn flush(&mut self) -> Result {
		Ok(self.ws.flush()?)
	}

	pub fn inner(&self) -> &WebSocket {
		&self.ws
	}
	pub fn inner_mut(&mut self) -> &mut WebSocket {
		&mut self.ws
	}
	pub fn into_inner(self) -> WebSocket {
		self.ws
	}
}
