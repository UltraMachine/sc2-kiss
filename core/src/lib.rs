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

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "launcher")]
pub mod launcher;

#[cfg(feature = "request-helpers")]
pub mod request;

pub mod common;
use common::{internal::*, *};

pub use sc2_prost;
use sc2_prost::{Request, Response};
use sc2_prost::{request::Request as RequestVar, response::Response as ResponseVar};
