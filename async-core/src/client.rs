use super::*;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{tungstenite::Error as WsError, MaybeTlsStream, WebSocketStream};

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
/// Core methods
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
	pub async fn connect(url: impl ToUrl + Unpin) -> Result<Self> {
		let (ws, _) = tokio_tungstenite::connect_async(url).await?;
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
		self.ws.send(req_into_msg(req)).await?;

		let Some(msg) = self.ws.next().await else {
			return Err(Error::WebSocket(WsError::AlreadyClosed));
		};
		let res = res_from_msg(msg?, req_kind)?;

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

	assert_eq!(client.status(), Status::Unset);

	let res = client.send(Req::Ping(Default::default())).await?;
	println!("Server Status: {:?}", res.status);

	assert_eq!(client.status(), Status::Launched);
	assert_eq!(client.status(), res.status);
	# Ok::<(), sc2_async_core::Error>(())
	# }).unwrap()
	```
	*/
	pub fn status(&self) -> Status {
		self.status
	}
}
