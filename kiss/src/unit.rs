use super::*;
use sc2_core::common::PlayerId;
use sc2_prost::unit_order::Target as PbOrderTarget;
use std::fmt;

pub use action::unit as action;
pub use sc2_prost::Unit as RawUnit;

/// Methods for extracting various data from unit
pub trait UnitExt {
	fn tag(&self) -> UnitTag;
	fn owner(&self) -> PlayerId;
	#[cfg(feature = "ids")]
	fn kind(&self) -> ids::UnitKind;
	#[cfg(feature = "ids")]
	fn buffs(&self) -> impl Iterator<Item = ids::Buff>;
	#[cfg(feature = "linalg")]
	fn pos2(&self) -> glam::Vec2;
	#[cfg(feature = "linalg")]
	fn pos3(&self) -> glam::Vec3;
	#[cfg(feature = "linalg")]
	fn ipos2(&self) -> glam::IVec2;
	#[cfg(feature = "linalg")]
	fn ipos3(&self) -> glam::IVec3;
}

impl UnitExt for RawUnit {
	fn tag(&self) -> UnitTag {
		self.tag.into()
	}
	fn owner(&self) -> PlayerId {
		self.owner.into()
	}
	#[cfg(feature = "ids")]
	fn kind(&self) -> ids::UnitKind {
		self.unit_type.into()
	}
	#[cfg(feature = "ids")]
	fn buffs(&self) -> impl Iterator<Item = ids::Buff> {
		self.buff_ids.iter().map(|&b| b.into())
	}
	#[cfg(feature = "linalg")]
	fn pos2(&self) -> glam::Vec2 {
		self.pos.map_or(glam::Vec2::ZERO, |p| p.as_vec2())
	}
	#[cfg(feature = "linalg")]
	fn pos3(&self) -> glam::Vec3 {
		self.pos.map_or(glam::Vec3::ZERO, Into::into)
	}
	#[cfg(feature = "linalg")]
	fn ipos2(&self) -> glam::IVec2 {
		self.pos.map_or(glam::IVec2::ZERO, |p| p.as_ivec2())
	}
	#[cfg(feature = "linalg")]
	fn ipos3(&self) -> glam::IVec3 {
		self.pos.map_or(glam::IVec3::ZERO, |p| p.as_ivec3())
	}
}

/**
Unique identifier for a unit.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UnitTag(pub u64);
impl fmt::Display for UnitTag {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl From<u64> for UnitTag {
	fn from(num: u64) -> UnitTag {
		UnitTag(num)
	}
}
impl From<UnitTag> for u64 {
	fn from(tag: UnitTag) -> u64 {
		tag.0
	}
}

pub trait OrderExt {
	fn ability(&self) -> ids::Ability;
	fn target<P>(&self) -> Option<action::Target<P>>
	where
		PbOrderTarget: Into<action::Target<P>>;
}
impl OrderExt for sc2_prost::UnitOrder {
	fn ability(&self) -> ids::Ability {
		self.ability_id.into()
	}
	fn target<P>(&self) -> Option<action::Target<P>>
	where
		PbOrderTarget: Into<action::Target<P>>,
	{
		self.target.map(Into::into)
	}
}
