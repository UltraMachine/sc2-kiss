#![allow(clippy::tabs_in_doc_comments)]

/*!
Asynchronous version of `sc2-core` crate.
Provides basic functionality to connect and communicate with SC2 instances.

Start by looking into [`Client`] documentation.
*/

use sc2_core::common::{internal::*, *};

pub use sc2_prost;

pub use sc2_core::common;

#[cfg(feature = "client")]
pub mod client;
#[doc(no_inline)]
#[cfg(feature = "client")]
pub use client::{Client, Error, Result};

#[cfg(feature = "request-helpers")]
pub use sc2_core::request;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "launcher")]
pub use sc2_core::launcher;
