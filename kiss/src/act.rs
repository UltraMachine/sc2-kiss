use super::*;
use std::collections::{HashMap, HashSet};

use sc2_core::{Client, Result};
use sc2_prost::{
	action_raw_unit_command::Target as PbTarget, unit_order::Target as PbOrderTarget,
	Action as PbAct, ActionRawUnitCommand as PbUnitCmd,
};

use ids::Ability;
use linalg::{IVec2, Vec2};
use unit::Tag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target<P = Vec2> {
	Pos(P),
	Unit(Tag),
}

impl From<Target> for PbTarget {
	fn from(target: Target) -> Self {
		match target {
			Target::Pos(pos) => PbTarget::WorldSpacePos(pos.into()),
			Target::Unit(tag) => PbTarget::UnitTag(tag.into()),
		}
	}
}
impl From<PbTarget> for Target {
	fn from(target: PbTarget) -> Self {
		match target {
			PbTarget::WorldSpacePos(pos) => Self::Pos(pos.into()),
			PbTarget::UnitTag(tag) => Self::Unit(tag.into()),
		}
	}
}

impl From<PbOrderTarget> for Target {
	fn from(target: PbOrderTarget) -> Self {
		match target {
			PbOrderTarget::WorldSpacePos(pos) => Self::Pos(pos.as_vec2()),
			PbOrderTarget::UnitTag(tag) => Self::Unit(tag.into()),
		}
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Act<P = Vec2> {
	pub ability: Ability,
	pub target: Option<Target<P>>,
	pub queue: bool,
}
impl<P> Act<P> {
	pub fn new(ability: Ability) -> Self {
		Self {
			ability,
			target: None,
			queue: false,
		}
	}
	pub fn target(mut self, target: Target<P>) -> Self {
		self.target = Some(target);
		self
	}
	pub fn target_none(mut self) -> Self {
		self.target = None;
		self
	}
	pub fn target_pos(self, pos: P) -> Self {
		self.target(Target::Pos(pos))
	}
	pub fn target_unit(self, tag: Tag) -> Self {
		self.target(Target::Unit(tag))
	}
	pub fn queue(mut self, queue: bool) -> Self {
		self.queue = queue;
		self
	}
}

type ActHashable = (Ability, Option<Tag>, bool);
type ActPos = (Ability, Vec2, bool);

#[derive(Debug, Default)]
pub struct Actions {
	acts: HashMap<ActHashable, HashSet<Tag>>,
	acts_pos: Vec<(ActPos, HashSet<Tag>)>,
}
impl Actions {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn is_empty(&self) -> bool {
		self.acts.is_empty() && self.acts_pos.is_empty()
	}
	pub fn add(&mut self, act: Act, tag: Tag) -> &mut Self {
		self.add_batch(act, [tag])
	}
	pub fn add_batch<I>(&mut self, act: Act, tags: I) -> &mut Self
	where
		I: IntoIterator<Item = Tag>,
	{
		let target = match act.target {
			None => None,
			Some(Target::Unit(tag)) => Some(tag),
			Some(Target::Pos(pos)) => {
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
	pub fn into_vec(self) -> Vec<PbAct> {
		self.into()
	}
	pub fn flush_to_vec(&mut self, buf: &mut Vec<PbAct>) {
		let it = self
			.acts
			.drain()
			.map(|((abil, target, queue), tags)| PbUnitCmd {
				ability_id: abil.into(),
				target: target.map(|tag| PbTarget::UnitTag(tag.into())),
				unit_tags: tags.into_iter().map(Into::into).collect(),
				queue_command: queue,
			})
			.chain(
				self.acts_pos
					.drain(..)
					.map(|((abil, pos, queue), tags)| PbUnitCmd {
						ability_id: abil.into(),
						target: Some(PbTarget::WorldSpacePos(pos.into())),
						unit_tags: tags.into_iter().map(Into::into).collect(),
						queue_command: queue,
					}),
			)
			.map(|cmd| PbAct {
				action_raw: Some(sc2_prost::ActionRaw {
					action: Some(sc2_prost::action_raw::Action::UnitCommand(cmd)),
				}),
				..<_>::default()
			});
		buf.extend(it);
	}
	pub fn flush(&mut self, client: &mut Client) -> Result {
		let mut buf = vec![];
		self.flush_to_vec(&mut buf);
		client.action(buf).map(|_| ())
	}
}

impl<TS> Extend<(Act, TS)> for Actions
where
	TS: IntoIterator<Item = Tag>,
{
	fn extend<I>(&mut self, iter: I)
	where
		I: IntoIterator<Item = (Act, TS)>,
	{
		for (act, tags) in iter {
			self.add_batch(act, tags);
		}
	}
}
impl<TS> FromIterator<(Act, TS)> for Actions
where
	TS: IntoIterator<Item = Tag>,
{
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = (Act, TS)>,
	{
		let mut acts = Self::new();
		acts.extend(iter);
		acts
	}
}

impl From<Actions> for Vec<PbAct> {
	fn from(mut acts: Actions) -> Self {
		let mut buf = vec![];
		acts.flush_to_vec(&mut buf);
		buf
	}
}

pub type ITarget = Target<IVec2>;
pub type IAct = Act<IVec2>;

impl From<ITarget> for PbTarget {
	fn from(target: ITarget) -> Self {
		match target {
			ITarget::Pos(pos) => PbTarget::WorldSpacePos((pos.as_vec2() + 0.5).into()),
			ITarget::Unit(tag) => PbTarget::UnitTag(tag.into()),
		}
	}
}
impl From<PbTarget> for ITarget {
	fn from(target: PbTarget) -> Self {
		match target {
			PbTarget::WorldSpacePos(pos) => Self::Pos(pos.as_ivec2()),
			PbTarget::UnitTag(tag) => Self::Unit(tag.into()),
		}
	}
}

impl From<PbOrderTarget> for ITarget {
	fn from(target: PbOrderTarget) -> Self {
		match target {
			PbOrderTarget::WorldSpacePos(pos) => Self::Pos(pos.as_ivec2()),
			PbOrderTarget::UnitTag(tag) => Self::Unit(tag.into()),
		}
	}
}

#[derive(Debug, Default, Clone)]
pub struct IActions(HashMap<IAct, HashSet<Tag>>);
impl IActions {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
	pub fn add(&mut self, act: IAct, tag: Tag) -> &mut Self {
		self.add_batch(act, [tag])
	}
	pub fn add_batch<I>(&mut self, act: IAct, tags: I) -> &mut Self
	where
		I: IntoIterator<Item = Tag>,
	{
		self.0.entry(act).or_default().extend(tags);
		self
	}
	pub fn into_vec(self) -> Vec<PbAct> {
		self.into()
	}
	pub fn flush_to_vec(&mut self, buf: &mut Vec<PbAct>) {
		let it = self
			.0
			.drain()
			.map(|(act, tags)| PbUnitCmd {
				ability_id: act.ability.into(),
				target: act.target.map(Into::into),
				unit_tags: tags.into_iter().map(Into::into).collect(),
				queue_command: act.queue,
			})
			.map(|cmd| PbAct {
				action_raw: Some(sc2_prost::ActionRaw {
					action: Some(sc2_prost::action_raw::Action::UnitCommand(cmd)),
				}),
				..<_>::default()
			});
		buf.extend(it);
	}
	pub fn flush(&mut self, client: &mut Client) -> Result {
		let mut buf = vec![];
		self.flush_to_vec(&mut buf);
		client.action(buf).map(|_| ())
	}
}

impl<TS> Extend<(IAct, TS)> for IActions
where
	TS: IntoIterator<Item = Tag>,
{
	fn extend<I>(&mut self, iter: I)
	where
		I: IntoIterator<Item = (IAct, TS)>,
	{
		for (act, tags) in iter {
			self.add_batch(act, tags);
		}
	}
}
impl<TS> FromIterator<(IAct, TS)> for IActions
where
	TS: IntoIterator<Item = Tag>,
{
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = (IAct, TS)>,
	{
		let mut acts = Self::new();
		acts.extend(iter);
		acts
	}
}

impl From<IActions> for Vec<PbAct> {
	fn from(mut acts: IActions) -> Self {
		let mut buf = vec![];
		acts.flush_to_vec(&mut buf);
		buf
	}
}
