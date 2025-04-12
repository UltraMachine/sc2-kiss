use super::*;

macro_rules! request {
	($client:ident . $Var:ident $(($req:expr))? $(.$field:ident)?) => {
		$client.request(Req::$Var(request!(@ $($req)?))).await.map_res(|res| {
			let ResVar::$Var(data) = res else {
				unreachable!()
			};
			data $(.$field)?
		})
	};
	(@) => { Default::default() };
	(@ $req:expr) => { $req };
}
pub(crate) use request;

pub mod setup;
pub use setup::*;

pub mod game;
pub use game::*;

pub mod other;
pub use other::*;

#[doc(no_inline)]
pub use common::*;
pub use sc2_core::request::common;
