use super::*;
use derive_more::From;
use glam::Vec2;
use std::collections::{HashMap, HashSet};

use ids::Ability;
use unit::Tag;

type ActHashable = (Ability, Option<Tag>, bool);
type ActPos = (Ability, Vec2, bool);

#[derive(Debug, Default)]
pub struct Actions {
	acts: HashMap<ActHashable, HashSet<Tag>>,
	acts_pos: Vec<(ActPos, Vec<Tag>)>,
}
impl Actions {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn add(&mut self, act: Act, tag: Tag) -> &mut Self {
		self.add_batch(act, [tag])
	}
	pub fn add_batch<I>(&mut self, act: Act, tags: I) -> &mut Self
	where
		I: IntoIterator<Item = Tag>,
	{
		let target = match act.target {
			Target::None => None,
			Target::Unit(tag) => Some(tag),
			Target::Pos(pos) => {
				self.acts_pos
					.push(((act.ability, pos, act.queue), tags.into_iter().collect()));
				return self;
			}
		};
		self.acts
			.entry((act.ability, target, act.queue))
			.or_default()
			.extend(tags);
		self
	}
}

type UnitCmd = sc2_prost::ActionRawUnitCommand;

impl From<Actions> for Vec<sc2_prost::Action> {
	fn from(acts: Actions) -> Self {
		acts.acts
			.into_iter()
			.map(|((abil, target, queue), tags)| UnitCmd {
				ability_id: abil.into(),
				target: target.map(|tag| ActTarget::UnitTag(tag.into())),
				unit_tags: tags.into_iter().map(Into::into).collect(),
				queue_command: queue,
			})
			.chain(
				acts.acts_pos
					.into_iter()
					.map(|((abil, pos, queue), tags)| UnitCmd {
						ability_id: abil.into(),
						target: Some(ActTarget::WorldSpacePos(pos.into())),
						unit_tags: tags.into_iter().map(Into::into).collect(),
						queue_command: queue,
					}),
			)
			.map(|cmd| sc2_prost::Action {
				action_raw: Some(sc2_prost::ActionRaw {
					action: Some(sc2_prost::action_raw::Action::UnitCommand(cmd)),
				}),
				..<_>::default()
			})
			.collect()
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Act {
	pub ability: Ability,
	pub target: Target,
	pub queue: bool,
}
impl Act {
	pub fn new(ability: Ability) -> Act {
		Act {
			ability,
			..<_>::default()
		}
	}
	pub fn target(mut self, target: Target) -> Act {
		self.target = target;
		self
	}
	pub fn target_pos(self, pos: Vec2) -> Act {
		self.target(Target::Pos(pos))
	}
	pub fn target_unit(self, tag: Tag) -> Act {
		self.target(Target::Unit(tag))
	}
	pub fn queue(mut self, queue: bool) -> Act {
		self.queue = queue;
		self
	}
}

use sc2_prost::action_raw_unit_command::Target as ActTarget;
use sc2_prost::unit_order::Target as OrderTarget;

#[derive(Debug, Default, From, Clone, Copy, PartialEq)]
pub enum Target {
	#[default]
	None,
	Pos(Vec2),
	Unit(Tag),
}

impl From<Target> for Option<ActTarget> {
	fn from(target: Target) -> Self {
		match target {
			Target::None => None,
			Target::Pos(pos) => Some(ActTarget::WorldSpacePos(pos.into())),
			Target::Unit(tag) => Some(ActTarget::UnitTag(tag.into())),
		}
	}
}
impl From<Option<ActTarget>> for Target {
	fn from(target: Option<ActTarget>) -> Self {
		target.map_or(Self::None, |t| match t {
			ActTarget::WorldSpacePos(pos) => Self::Pos(pos.into()),
			ActTarget::UnitTag(tag) => Self::Unit(tag.into()),
		})
	}
}
impl From<Option<OrderTarget>> for Target {
	fn from(target: Option<OrderTarget>) -> Self {
		target.map_or(Self::None, |t| match t {
			OrderTarget::WorldSpacePos(pos) => Self::Pos(glam::Vec3::from(pos).truncate()),
			OrderTarget::UnitTag(tag) => Self::Unit(tag.into()),
		})
	}
}
