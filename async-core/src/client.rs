use super::*;
use futures_util::{SinkExt, StreamExt};
use std::{io, net::ToSocketAddrs};
use tokio_tungstenite::{
	MaybeTlsStream, WebSocketStream,
	tungstenite::{Error as WsError, client::IntoClientRequest, http},
};

type WebSocket = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

#[doc(no_inline)]
pub use sc2_core::{Error, Result};

/**
Asynchronous client interface to connect and communicate with SC2 instance.

Basic usage:
```no_run
use sc2_async_core::{Client, Req};

let mut client = Client::connect("localhost:5000").await?;
let res = client.request(Req::Ping(Default::default())).await?;
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
	async fn connect_imp(http_req: http::Request<()>) -> Result<Self> {
		let (ws, _) = tokio_tungstenite::connect_async(http_req).await?;
		Ok(Self {
			ws,
			status: Status::Unset,
		})
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
		let addrs = addr.to_socket_addrs().map_err(WsError::Io)?;
		let mut last_err = None;
		for addr in addrs {
			let http_req = format!("ws://{addr}/sc2api").into_client_request()?;

			match Self::connect_imp(http_req).await {
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
	use sc2_async_core::{Req, ResVar};

	let res = client.request(Req::Ping(Default::default())).await?;
	println!("Server Status: {:?}", res.status);
	let ResVar::Ping(data) = res.data else { unreachable!() };
	println!("Game Version: {}", data.game_version);
	println!("Data Version: {}", data.data_version);
	println!("Data Build: {}", data.data_build);
	println!("Base Build: {}", data.base_build);
	```
	```no_run
	use sc2_async_core::{Req, ResVar};

	let res = client.request(Req::AvailableMaps(Default::default())).await?;
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
	use sc2_async_core::{Req, ResVar};

	let req = sc2_prost::RequestJoinGame {
		// Set your options here
		..Default::default()
	};
	let res = client.request(Req::JoinGame(req)).await?;
	println!("Server Status: {:?}", res.status);
	let ResVar::JoinGame(data) = res.data else { unreachable!() };
	println!("Our Player Id: {}", data.player_id);
	```
	*/
	pub async fn request(&mut self, req: Req) -> Result<Res> {
		let req_kind = req.kind();
		self.send(req).await?;
		self.read(req_kind).await
	}

	pub async fn send(&mut self, req: Req) -> Result {
		self.ws.send(req_into_msg(req)).await.map_err(Into::into)
	}
	pub async fn read(&mut self, expect_kind: Kind) -> Result<Res> {
		let Some(msg) = self.ws.next().await else {
			return Err(Error::WebSocket(WsError::AlreadyClosed));
		};
		let res = res_from_msg(msg?, expect_kind)?;
		check_res(res, expect_kind, &mut self.status)
	}

	/**
	Returns current server [`Status`].

	# Examples
	```no_run
	use sc2_async_core::{Req, Status};

	assert_eq!(client.status(), Status::Unset);

	let res = client.request(Req::Ping(Default::default())).await?;
	println!("Server Status: {:?}", res.status);

	assert_eq!(client.status(), Status::Launched);
	assert_eq!(client.status(), res.status);
	```
	*/
	pub fn status(&self) -> Status {
		self.status
	}
}
