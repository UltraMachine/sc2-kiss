use super::*;

pub use sc2_prost::Action;

pub trait ToActions {
	fn to_iter(self) -> impl Iterator<Item = Action>;
	fn to_actions(self) -> Vec<Action>
	where
		Self: Sized,
	{
		self.to_iter().collect()
	}
}

impl ToActions for () {
	fn to_iter(self) -> impl Iterator<Item = Action> {
		[].into_iter()
	}
	fn to_actions(self) -> Vec<Action> {
		vec![]
	}
}

#[cfg(feature = "unit")]
pub mod unit;

#[cfg(feature = "chat")]
pub mod chat;
