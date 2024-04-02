include!(concat!(env!("OUT_DIR"), "/sc2api_protocol.rs"));

#[cfg(feature = "serde")]
mod srd {
	use super::*;
	use serde::{Deserialize, Deserializer, Serialize, Serializer};

	macro_rules! serde_enums {
		($($mod:ident => $enum:ty),+ $(,)?) => {$(
			pub mod $mod {
				use super::*;
				pub fn serialize<S: Serializer>(num: &i32, s: S) -> Result<S::Ok, S::Error> {
					<$enum>::try_from(*num)
						.unwrap_or_default()
						.serialize(s)
				}
				pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<i32, D::Error> {
					<$enum>::deserialize(d).map(|res| res as i32)
				}
			}
		)+};
	}
	serde_enums! {
		target => ability_data::Target,
		race => Race,
		attribute => Attribute,
		weapon_target => weapon::TargetType,
	}
	pub mod attributes {
		use super::*;
		pub fn serialize<S: Serializer>(nums: &[i32], s: S) -> Result<S::Ok, S::Error> {
			nums.iter()
				.map(|num| Attribute::try_from(*num).unwrap_or_default())
				.collect::<Vec<_>>()
				.serialize(s)
		}
		pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<i32>, D::Error> {
			Vec::<Attribute>::deserialize(d)
				.map(|res| res.into_iter().map(|item| item as i32).collect())
		}
	}
}

#[cfg(feature = "glam")]
mod glam {
	use crate::{Point, Point2D};
	use glam::{Vec2, Vec3};

	macro_rules! from {
		(@ $T:ident[$($f:ident),+] = $F:ident) => {
			impl From<$F> for $T {
				fn from(f: $F) -> $T {
					$T { $($f: f.$f),+ }
				}
			}
		};
		($($T:ident[$($f:ident),+] = $F:ident)+) => {$(
			from!(@ $T[$($f),+] = $F);
			from!(@ $F[$($f),+] = $T);
		)+};
	}
	from! {
		Vec2[x,y] = Point2D
		Vec3[x,y,z] = Point
	}
}
