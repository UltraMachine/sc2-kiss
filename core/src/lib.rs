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
- [x] Split send and recieve
- [ ] Update async
- [x] Auto locate game dir
*/

use thiserror::Error;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "client")]
pub use client::{Client, Error, Result};

// #[cfg(feature = "server")]
// pub mod server;
// #[cfg(feature = "server")]
// pub use server::Server;

#[cfg(feature = "instance")]
pub mod instance;
#[cfg(feature = "instance")]
pub use instance::Launcher;

#[cfg(feature = "request-helpers")]
pub mod request;

pub mod common;
use common::{internal::*, *};

pub use sc2_prost::{self, Status, request::Request as Req, response::Response as ResVar};
