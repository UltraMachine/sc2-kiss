use super::Ability;
use sc2_core::{Client, Result, request::data};
use std::collections::HashMap;

/// Remaps specific ability ids to their general versions
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AbilityRemap(HashMap<Ability, Ability>);

impl AbilityRemap {
	pub fn new(map: HashMap<Ability, Ability>) -> Self {
		Self(map)
	}
	/// Initializes the map with already known list of `AbilityData`
	pub fn with_data<'a, I>(data: I) -> Self
	where
		I: IntoIterator<Item = &'a sc2_prost::AbilityData>,
	{
		Self(
			data.into_iter()
				.filter(|abil| abil.available && abil.remaps_to_ability_id != 0)
				.map(|abil| (Ability(abil.ability_id), Ability(abil.remaps_to_ability_id)))
				.collect(),
		)
	}
	/// Requests data from API and initializes the map with it
	pub fn with_client(client: &mut Client) -> Result<Self> {
		let res = client.request(data().abilities(true))?;
		Ok(Self::with_data(&res.data.abilities))
	}
	/// Returns generalized version of input ability, or itself if ability is not in the map
	pub fn remap(&self, id: Ability) -> Ability {
		self.try_remap(id).unwrap_or(id)
	}
	pub fn try_remap(&self, id: Ability) -> Option<Ability> {
		self.0.get(&id).copied()
	}
}
