#[cfg(feature = "serde")]
mod srd {
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
		target => crate::ability_data::Target,
		race => crate::Race,
		attribute => crate::Attribute,
		weapon_target => crate::weapon::TargetType,
	}
	pub mod attributes {
		use super::*;
		pub fn serialize<S: Serializer>(nums: &[i32], s: S) -> Result<S::Ok, S::Error> {
			nums.iter()
				.map(|num| crate::Attribute::try_from(*num).unwrap_or_default())
				.collect::<Vec<_>>()
				.serialize(s)
		}
		pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<i32>, D::Error> {
			Vec::<crate::Attribute>::deserialize(d)
				.map(|res| res.into_iter().map(|item| item as i32).collect())
		}
	}
}

include!(concat!(env!("OUT_DIR"), "/sc2api_protocol.rs"));
