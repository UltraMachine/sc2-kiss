#![allow(clippy::tabs_in_doc_comments)]

/*!
Asynchronous version of `sc2-core` crate.
Provides basic functionality to connect and communicate with SC2 instances.

Start by looking into [`Client`] documentation.
*/

use sc2_core::common::{internal::*, *};

pub mod client;
#[cfg(feature = "request-methods")]
pub mod request;

#[doc(inline)]
pub use client::Client;
pub use sc2_core::*;
