use prost::DecodeError;
use sc2_prost::{Request, Response};
use std::{
	io,
	net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
};
use thiserror::Error;
use tungstenite::{
	Error as WsError, HandshakeError, ServerHandshake,
	handshake::server::{
		Callback, ErrorResponse, Request as ServerRequest, Response as ServerResponse,
	},
	http::StatusCode,
};

use crate::common::internal::{req_from_msg, res_into_msg};

type WS = tungstenite::WebSocket<TcpStream>;
type HsError = HandshakeError<ServerHandshake<TcpStream, ServerCallback>>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("OS error: {0}")]
	Io(#[from] io::Error),
	#[error("Handshake error: {0}")]
	Handshake(Box<HsError>),
	#[error("WebSocket error: {0}")]
	WebSocket(#[from] WsError),
	#[error("Decode error: {0}")]
	Decode(#[from] DecodeError),
}
impl From<HsError> for Error {
	fn from(e: HsError) -> Self {
		Self::Handshake(Box::new(e))
	}
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Server(TcpListener);
impl Server {
	pub fn bind(addr: impl ToSocketAddrs) -> Result<Self> {
		Ok(Self(TcpListener::bind(addr)?))
	}
	pub fn local_addr(&self) -> Result<SocketAddr> {
		Ok(self.0.local_addr()?)
	}
	pub fn try_clone(&self) -> Result<Self> {
		Ok(Self(self.0.try_clone()?))
	}
	pub fn accept(&self) -> Result<(ServerClient, SocketAddr)> {
		let (socket, addr) = self.0.accept()?;
		Ok((socket.try_into()?, addr))
	}
	pub fn incoming(&self) -> impl Iterator<Item = Result<ServerClient>> {
		self.0.incoming().map(|socket| Ok(socket?.try_into()?))
	}

	pub fn inner(&self) -> &TcpListener {
		&self.0
	}
	pub fn into_inner(self) -> TcpListener {
		self.0
	}
}

#[derive(Debug)]
pub struct ServerClient(WS);
impl ServerClient {
	pub fn read(&mut self) -> Result<Request> {
		req_from_msg(self.0.read()?)
	}
	pub fn send(&mut self, res: Response) -> Result {
		Ok(self.0.send(res_into_msg(res))?)
	}

	pub fn write(&mut self, res: Response) -> Result {
		Ok(self.0.write(res_into_msg(res))?)
	}
	pub fn flush(&mut self) -> Result {
		Ok(self.0.flush()?)
	}

	pub fn inner(&self) -> &WS {
		&self.0
	}
	pub fn inner_mut(&mut self) -> &mut WS {
		&mut self.0
	}
	pub fn into_inner(self) -> WS {
		self.0
	}
}
impl TryFrom<TcpStream> for ServerClient {
	type Error = HsError;
	fn try_from(socket: TcpStream) -> Result<Self, Self::Error> {
		Ok(Self(tungstenite::accept_hdr(socket, ServerCallback)?))
	}
}

#[derive(Debug)]
pub struct ServerCallback;
impl Callback for ServerCallback {
	fn on_request(
		self,
		request: &ServerRequest,
		response: ServerResponse,
	) -> Result<ServerResponse, ErrorResponse> {
		if request.uri().path() != "/sc2api" {
			return Err(ServerResponse::builder()
				.status(StatusCode::FORBIDDEN)
				.body(None)
				.unwrap());
		}
		Ok(response)
	}
}
