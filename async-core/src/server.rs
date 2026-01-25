use futures_util::{SinkExt, StreamExt};
use sc2_prost::{Request, Response};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_tungstenite::{WebSocketStream, accept_hdr_async, tungstenite::Error as WsError};

use crate::common::internal::{req_from_msg, res_into_msg};

pub use sc2_core::server::{Error, Result, ServerCallback};

type WebSocket = WebSocketStream<TcpStream>;

#[derive(Debug)]
pub struct Server(TcpListener);
impl Server {
	pub async fn bind(addr: impl ToSocketAddrs) -> Result<Self> {
		Ok(Self(TcpListener::bind(addr).await?))
	}

	pub async fn accept(&self) -> Result<(ServerClient, SocketAddr)> {
		let (stream, addr) = self.0.accept().await?;
		let client = ServerClient::try_from_stream(stream).await?;
		Ok((client, addr))
	}

	pub fn local_addr(&self) -> Result<SocketAddr> {
		Ok(self.0.local_addr()?)
	}

	pub fn inner(&self) -> &TcpListener {
		&self.0
	}
	pub fn into_inner(self) -> TcpListener {
		self.0
	}
}

#[derive(Debug)]
pub struct ServerClient(WebSocket);
impl ServerClient {
	pub async fn try_from_stream(stream: TcpStream) -> Result<Self> {
		Ok(Self(accept_hdr_async(stream, ServerCallback).await?))
	}

	pub async fn read(&mut self) -> Result<Request> {
		let Some(msg) = self.0.next().await else {
			return Err(WsError::AlreadyClosed.into());
		};
		req_from_msg(msg?)
	}

	pub async fn send(&mut self, res: Response) -> Result {
		self.write(res).await?;
		self.flush().await
	}

	pub async fn write(&mut self, res: Response) -> Result {
		let msg = res_into_msg(res);
		Ok(self.0.feed(msg).await?)
	}

	pub async fn flush(&mut self) -> Result {
		Ok(self.0.flush().await?)
	}

	pub fn inner(&self) -> &WebSocket {
		&self.0
	}
	pub fn inner_mut(&mut self) -> &mut WebSocket {
		&mut self.0
	}
	pub fn into_inner(self) -> WebSocket {
		self.0
	}
}
