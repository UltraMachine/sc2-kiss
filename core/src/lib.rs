#![allow(clippy::tabs_in_doc_comments)]

/*!
Core crate of `sc2-kiss` framework.
Provides basic functionality to connect and communicate with SC2 instances.

Start by looking into [`Client`] documentation.

## Todo:
- [x] Client connecting and request sending
- [x] Check response kind/status/errors
- [x] Optional methods for easier request making
- [x] SC2 Instance launching
- [ ] Proxy between client and sc2
*/

use thiserror::Error;

pub mod client;
pub use client::{Client, Error, Result};
// mod server;
// pub use server::Server;
pub mod instance;

pub mod common;
#[cfg(feature = "request-methods")]
pub mod request;

use common::{internal::*, *};

pub use sc2_prost::{self, request::Request as Req, response::Response as ResVar, Status};
pub use tungstenite::client::IntoClientRequest as ToUrl;
