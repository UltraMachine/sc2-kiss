use super::*;
use sc2_prost::Unit;
use std::fmt;

/// Methods for extracting various data from unit
pub trait UnitExt {
	fn tag(&self) -> Tag;
	#[cfg(feature = "ids")]
	fn kind(&self) -> ids::UnitKind;
	#[cfg(feature = "linalg")]
	fn pos2(&self) -> glam::Vec2;
	#[cfg(feature = "linalg")]
	fn pos3(&self) -> glam::Vec3;
}
impl UnitExt for Unit {
	fn tag(&self) -> Tag {
		self.tag.into()
	}
	#[cfg(feature = "ids")]
	fn kind(&self) -> ids::UnitKind {
		self.unit_type.into()
	}
	#[cfg(feature = "linalg")]
	fn pos2(&self) -> glam::Vec2 {
		self.pos3().truncate()
	}
	#[cfg(feature = "linalg")]
	fn pos3(&self) -> glam::Vec3 {
		self.pos.clone().map_or(glam::Vec3::ZERO, Into::into)
	}
}

/**
Unique identifier for a unit.

`u64` wrapper which provides distinct type name and
doesn't allow math operations or mutation of inner value.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Tag(u64);
impl Tag {
	pub fn num(self) -> u64 {
		self.0
	}
}
impl fmt::Display for Tag {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl From<u64> for Tag {
	fn from(num: u64) -> Tag {
		Tag(num)
	}
}
impl From<Tag> for u64 {
	fn from(tag: Tag) -> u64 {
		tag.0
	}
}
