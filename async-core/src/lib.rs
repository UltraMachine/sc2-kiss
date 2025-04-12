#![allow(clippy::tabs_in_doc_comments)]

/*!
Asynchronous version of `sc2-core` crate.
Provides basic functionality to connect and communicate with SC2 instances.

Start by looking into [`Client`] documentation.
*/

use sc2_core::common::{internal::*, *};

pub use sc2_prost;

pub use sc2_core::{Launcher, Req, ResVar, Status, common, instance};

pub mod client;
#[doc(no_inline)]
pub use client::{Client, Error, Result};

#[cfg(feature = "request-helpers")]
pub mod request;
