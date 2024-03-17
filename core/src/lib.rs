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

pub mod client;
pub mod common;
#[cfg(feature = "request-methods")]
pub mod request;

use common::{internal::*, *};

#[doc(inline)]
pub use client::Client;
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
