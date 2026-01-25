use super::*;
use futures_util::{SinkExt, StreamExt};
use sc2_prost::{Request, Response};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio_tungstenite::{WebSocketStream, client_async, tungstenite::Error as WsError};

type WebSocket = WebSocketStream<TcpStream>;

#[doc(no_inline)]
pub use sc2_core::{Error, Result};

/**
Asynchronous client interface to connect and communicate with SC2 instance.

Basic usage:
```no_run
use sc2_async_core::{Client, request::Ping};

let mut client = Client::connect("localhost:5000").await?;
let res = client.request(Ping).await?;
println!("{res:?}");
```
*/
#[derive(Debug)]
pub struct Client {
	ws: WebSocket,
}
/// Core methods
impl Client {
	async fn _connect(stream: TcpStream) -> Result<Self> {
		stream.set_nodelay(true)?;
		let req = format!("ws://{}/sc2api", stream.peer_addr()?);
		let (ws, _) = client_async(req, stream).await?;
		Ok(Self { ws })
	}

	/**
	Connects to the given SC2API WebSocket server and returns [`Client`].

	# Errors

	This function can error in case of invalid URL or connection failure.

	# Examples

	```no_run
	use sc2_async_core::Client;

	let mut client = Client::connect("localhost:5000").await?;
	```
	*/
	pub async fn connect(addr: impl ToSocketAddrs) -> Result<Self> {
		Self::_connect(TcpStream::connect(addr).await?).await
	}

	/**
	Sends given request and returns received response.

	# Errors

	This method can return error in the following cases:
	- WebSocket failure
	- Failed to decode response
	- Response [`Kind`] doesn't match request [`Kind`]
	- Response contains any errors

	# Examples
	```no_run
	use sc2_async_core::request::Ping;

	let res = client.request(Ping).await?;
	println!("Server Status: {:?}", res.status);
	let data = res.data;
	println!("Game Version: {}", data.game_version);
	println!("Data Version: {}", data.data_version);
	println!("Data Build: {}", data.data_build);
	println!("Base Build: {}", data.base_build);
	```
	```no_run
	use sc2_async_core::request::AvailableMaps;

	let res = client.request(AvailableMaps).await?;
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
	use sc2_async_core::{
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
	let res = client.request(req).await?;
	println!("Server Status: {:?}", res.status);
	println!("Our Player Id: {}", res.data.player_id);
	```
	*/
	pub async fn request<R>(&mut self, request: R) -> Result<R::Output>
	where
		R: Into<Request> + ParseResponse,
	{
		self.send(request).await?;
		self.read::<R>().await
	}

	async fn _read(&mut self) -> Result<Response> {
		let Some(msg) = self.ws.next().await else {
			return Err(WsError::AlreadyClosed.into());
		};
		res_from_msg(msg?)
	}
	pub async fn read<R: ParseResponse>(&mut self) -> Result<R::Output> {
		R::parse(self._read().await?)
	}

	pub async fn send(&mut self, request: impl Into<Request>) -> Result {
		self.write(request).await?;
		self.flush().await
	}

	async fn _write(&mut self, req: Request) -> Result {
		let msg = req_into_msg(req);
		Ok(self.ws.feed(msg).await?)
	}
	pub async fn write(&mut self, request: impl Into<Request>) -> Result {
		self._write(request.into()).await
	}

	pub async fn flush(&mut self) -> Result {
		Ok(self.ws.flush().await?)
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
