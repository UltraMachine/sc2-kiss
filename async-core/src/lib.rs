#![allow(clippy::tabs_in_doc_comments)]

/*!
Asynchronous version of `sc2-core` crate.
Provides basic functionality to connect and communicate with SC2 instances.

Start by looking into [`Client`] documentation.
*/

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{tungstenite::Error as WsError, MaybeTlsStream, WebSocketStream};

use sc2_core::common::{internal::*, *};

pub use sc2_core::{Error, Req, ResVar, Result, Status, ToUrl};

type WebSocket = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

/**
Asynchronous client interface to connect and communicate with SC2 instance.

Basic usage:
```
# tokio::runtime::Builder::new_current_thread().enable_all().build()
# .unwrap().block_on(async {
use sc2_async_core::{Client, Req};

let mut client = Client::connect("ws://localhost:5000/sc2api").await?;
let res = client.send(Req::Ping(Default::default())).await?;
println!("{res:?}");
# Ok::<(), sc2_async_core::Error>(())
# }).unwrap()
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
	# tokio::runtime::Builder::new_current_thread().enable_all().build()
	# .unwrap().block_on(async {
	use sc2_async_core::Client;

	let mut client = Client::connect("ws://127.0.0.1:5000/sc2api").await?;
	# Ok::<(), sc2_async_core::Error>(())
	# }).unwrap()
	```
	```
	# tokio::runtime::Builder::new_current_thread().enable_all().build()
	# .unwrap().block_on(async {
	use sc2_async_core::Client;
	use url::Url;

	let url = Url::parse("ws://127.0.0.1:5000/sc2api").unwrap();
	let mut client = Client::connect(url).await?;
	# Ok::<(), sc2_async_core::Error>(())
	# }).unwrap()
	```
	```
	# tokio::runtime::Builder::new_current_thread().enable_all().build()
	# .unwrap().block_on(async {
	use sc2_async_core::Client;
	use http::Uri;

	let uri = "ws://127.0.0.1:5000/sc2api".parse::<Uri>().unwrap();
	let mut client = Client::connect(uri).await?;
	# Ok::<(), sc2_async_core::Error>(())
	# }).unwrap()
	```
	*/
	pub async fn connect(to: impl ToUrl + Unpin) -> Result<Self> {
		let (mut ws, _) = tokio_tungstenite::connect_async(to).await?;
		let res = send_no_check(&mut ws, Req::Ping(Default::default())).await?;
		Ok(Self {
			ws,
			status: res.status,
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
	# tokio::runtime::Builder::new_current_thread().enable_all().build()
	# .unwrap().block_on(async {
	# let mut client = sc2_async_core::Client::connect("ws://localhost:5000/sc2api").await?;
	use sc2_async_core::{Req, ResVar};

	let res = client.send(Req::Ping(Default::default())).await?;
	println!("Server Status: {:?}", res.status);
	let ResVar::Ping(data) = res.data else { unreachable!() };
	println!("Game Version: {}", data.game_version);
	println!("Data Version: {}", data.data_version);
	println!("Data Build: {}", data.data_build);
	println!("Base Build: {}", data.base_build);
	# Ok::<(), sc2_async_core::Error>(())
	# }).unwrap()
	```
	```
	# tokio::runtime::Builder::new_current_thread().enable_all().build()
	# .unwrap().block_on(async {
	# let mut client = sc2_async_core::Client::connect("ws://localhost:5000/sc2api").await?;
	use sc2_async_core::{Req, ResVar};

	let res = client.send(Req::AvailableMaps(Default::default())).await?;
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
	# Ok::<(), sc2_async_core::Error>(())
	# }).unwrap()
	```
	```
	# tokio::runtime::Builder::new_current_thread().enable_all().build()
	# .unwrap().block_on(async {
	# let mut client = sc2_async_core::Client::connect("ws://localhost:5000/sc2api").await?;
	use sc2_async_core::{Req, ResVar};

	let req = sc2_prost::RequestJoinGame {
		// Set your options here
		..Default::default()
	};
	let res = client.send(Req::JoinGame(req)).await?;
	println!("Server Status: {:?}", res.status);
	let ResVar::JoinGame(data) = res.data else { unreachable!() };
	println!("Our Player Id: {}", data.player_id);
	# Ok::<(), sc2_async_core::Error>(())
	# }).unwrap()
	```
	*/
	pub async fn send(&mut self, req: Req) -> Result<Res> {
		let req_kind = req.kind();
		let res = send_no_check(&mut self.ws, req).await?;
		check_res(res, req_kind, &mut self.status)
	}

	/**
	Returns current server [`Status`].

	# Examples
	```
	# tokio::runtime::Builder::new_current_thread().enable_all().build()
	# .unwrap().block_on(async {
	# let mut client = sc2_async_core::Client::connect("ws://localhost:5000/sc2api").await?;
	use sc2_async_core::{Req, Status};

	assert_eq!(client.status(), Status::Launched);

	let res = client.send(Req::Ping(Default::default())).await?;
	println!("Server Status: {:?}", res.status);

	assert_eq!(client.status(), res.status);
	# Ok::<(), sc2_async_core::Error>(())
	# }).unwrap()
	```
	*/
	pub fn status(&self) -> Status {
		self.status
	}
}

async fn send_no_check(ws: &mut WebSocket, req: Req) -> Result<Res> {
	ws.send(req_into_msg(req)).await?;

	let Some(msg) = ws.next().await else {
		return Err(Error::WebSocket(WsError::AlreadyClosed));
	};
	res_from_msg(msg?)
}
