use super::*;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

use sc2_prost::{
	Action, action_raw_unit_command::Target as PbTarget, unit_order::Target as PbOrderTarget,
};
use sc2_prost::{Point, Point2D};

use crate::unit::Tag;
use ids::Ability;
use linalg::{IVec2, Vec2, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target<P> {
	Pos(P),
	Unit(Tag),
}

pub type Target2 = Target<Vec2>;
pub type ITarget2 = Target<IVec2>;
pub type Target3 = Target<Vec3>;

impl<P: fmt::Display> fmt::Display for Target<P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Target::Pos(pos) => write!(f, "{pos}"),
			Target::Unit(tag) => write!(f, "{tag}"),
		}
	}
}

impl<P> From<Tag> for Target<P> {
	fn from(tag: Tag) -> Self {
		Self::Unit(tag)
	}
}
impl From<Vec2> for Target2 {
	fn from(pos: Vec2) -> Self {
		Self::Pos(pos)
	}
}
impl From<IVec2> for ITarget2 {
	fn from(pos: IVec2) -> Self {
		Self::Pos(pos)
	}
}
impl From<Vec3> for Target3 {
	fn from(pos: Vec3) -> Self {
		Self::Pos(pos)
	}
}

pub trait ToTargetPos {
	fn to_pos(self) -> Point2D;
}

impl ToTargetPos for Vec2 {
	fn to_pos(self) -> Point2D {
		self.into()
	}
}
impl ToTargetPos for IVec2 {
	fn to_pos(self) -> Point2D {
		(self.as_vec2() + 0.5).into()
	}
}

impl<P> From<Target<P>> for PbTarget
where
	P: ToTargetPos,
{
	fn from(target: Target<P>) -> Self {
		match target {
			Target::Pos(pos) => PbTarget::WorldSpacePos(pos.to_pos()),
			Target::Unit(tag) => PbTarget::UnitTag(tag.into()),
		}
	}
}

pub trait FromOrderPos {
	fn from_pos(pos: Point) -> Self;
}

impl FromOrderPos for Vec3 {
	fn from_pos(pos: Point) -> Self {
		pos.into()
	}
}
impl FromOrderPos for Vec2 {
	fn from_pos(pos: Point) -> Self {
		pos.as_vec2()
	}
}
impl FromOrderPos for IVec2 {
	fn from_pos(pos: Point) -> Self {
		pos.as_ivec2()
	}
}

impl<P: FromOrderPos> From<PbOrderTarget> for Target<P> {
	fn from(target: PbOrderTarget) -> Self {
		match target {
			PbOrderTarget::WorldSpacePos(pos) => Self::Pos(P::from_pos(pos)),
			PbOrderTarget::UnitTag(tag) => Self::Unit(tag.into()),
		}
	}
}

pub trait ToTarget<P> {
	fn to_target(self) -> Option<Target<P>>;
}

impl<P> ToTarget<P> for () {
	fn to_target(self) -> Option<Target<P>> {
		None
	}
}
impl<T, P> ToTarget<P> for T
where
	T: Into<Target<P>>,
{
	fn to_target(self) -> Option<Target<P>> {
		Some(self.into())
	}
}
impl<T, P> ToTarget<P> for Option<T>
where
	T: Into<Target<P>>,
{
	fn to_target(self) -> Option<Target<P>> {
		self.map(Into::into)
	}
}

impl Ability {
	pub fn target<P>(self, target: impl ToTarget<P>) -> UnitAction<P> {
		UnitAction {
			ability: self,
			target: target.to_target(),
		}
	}
	pub fn queue<P>(self, queue: bool) -> UnitActions<P> {
		UnitActions {
			actions: vec![self.into()],
			queue,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitAction<P> {
	pub ability: Ability,
	pub target: Option<Target<P>>,
}

impl<P> UnitAction<P> {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn queue(self, queue: bool) -> UnitActions<P> {
		UnitActions {
			actions: vec![self],
			queue,
		}
	}
}

// note: derive forces unnecessary bound (P: Default)
impl<P> Default for UnitAction<P> {
	fn default() -> Self {
		Self {
			ability: Ability::NONE,
			target: None,
		}
	}
}
impl<P> From<()> for UnitAction<P> {
	fn from(_: ()) -> Self {
		Self::default()
	}
}
impl<P> From<Ability> for UnitAction<P> {
	fn from(ability: Ability) -> Self {
		Self {
			ability,
			target: None,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitActions<P> {
	actions: Vec<UnitAction<P>>,
	queue: bool,
}
impl<P> UnitActions<P> {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn queue(mut self, queue: bool) -> Self {
		self.queue = queue;
		self
	}
}

// note: derive forces unnecessary bound (P: Default)
impl<P> Default for UnitActions<P> {
	fn default() -> Self {
		Self {
			actions: vec![],
			queue: false,
		}
	}
}
impl<P> From<()> for UnitActions<P> {
	fn from(_: ()) -> Self {
		Self::default()
	}
}
impl<P> From<Ability> for UnitActions<P> {
	fn from(ability: Ability) -> Self {
		Self {
			actions: vec![ability.into()],
			queue: false,
		}
	}
}
impl<P> From<UnitAction<P>> for UnitActions<P> {
	fn from(action: UnitAction<P>) -> Self {
		Self {
			actions: vec![action],
			queue: false,
		}
	}
}
impl<P> FromIterator<UnitAction<P>> for UnitActions<P> {
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = UnitAction<P>>,
	{
		Self {
			actions: <_>::from_iter(iter),
			queue: false,
		}
	}
}
impl<P> Extend<UnitAction<P>> for UnitActions<P> {
	fn extend<I>(&mut self, iter: I)
	where
		I: IntoIterator<Item = UnitAction<P>>,
	{
		self.actions.extend(iter);
	}
}

#[macro_export]
macro_rules! actions {
	() => {
		$crate::action::unit::UnitActions::default()
	};
	($($action:expr),+ $(,)?) => {
		$crate::action::unit::UnitActions::from_iter([$($crate::action::unit::UnitAction::from($action)),+])
	};
}
pub use actions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitsActions<P> {
	actions: HashMap<Tag, UnitActions<P>>,
}
impl<P> UnitsActions<P> {
	pub fn new() -> Self {
		Self::default()
	}
}

// note: derive forces unnecessary bound (P: Default)
impl<P> Default for UnitsActions<P> {
	fn default() -> Self {
		Self {
			actions: Default::default(),
		}
	}
}
impl<P> From<()> for UnitsActions<P> {
	fn from(_: ()) -> Self {
		Self::default()
	}
}

impl<P> FromIterator<(Tag, UnitActions<P>)> for UnitsActions<P> {
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = (Tag, UnitActions<P>)>,
	{
		Self {
			actions: <_>::from_iter(iter),
		}
	}
}
impl<P> Extend<(Tag, UnitActions<P>)> for UnitsActions<P> {
	fn extend<I>(&mut self, iter: I)
	where
		I: IntoIterator<Item = (Tag, UnitActions<P>)>,
	{
		self.actions.extend(iter);
	}
}

fn raw_units_action(
	tags: Vec<u64>,
	ability: Ability,
	target: Option<PbTarget>,
	queue: bool,
) -> Action {
	let cmd = sc2_prost::action_raw::Action::UnitCommand(sc2_prost::ActionRawUnitCommand {
		ability_id: ability.0,
		unit_tags: tags,
		queue_command: queue,
		target,
	});
	Action {
		action_raw: Some(sc2_prost::ActionRaw { action: Some(cmd) }),
		..Default::default()
	}
}

impl ToActions for UnitsActions<IVec2> {
	fn to_iter(self) -> impl Iterator<Item = Action> {
		type Actions = HashMap<UnitAction<IVec2>, HashSet<Tag>>;
		let mut force_actions = Actions::default();
		let mut queue_actions = Vec::<Actions>::default();

		for (tag, unit_actions) in self.actions {
			let mut it = unit_actions.actions.into_iter();

			if !unit_actions.queue {
				let Some(action) = it.next() else { continue };
				force_actions.entry(action).or_default().insert(tag);
			}

			for (i, action) in (0..).zip(it) {
				if let Some(q_actions) = queue_actions.get_mut(i) {
					q_actions.entry(action).or_default().insert(tag);
				} else {
					queue_actions.push([(action, [tag].into())].into());
				}
			}
		}

		force_actions
			.into_iter()
			.map(|(action, tags)| (action, tags, false))
			.chain(
				queue_actions
					.into_iter()
					.flatten()
					.map(|(action, tags)| (action, tags, true)),
			)
			.map(|(action, tags, queue)| {
				raw_units_action(
					tags.into_iter().map(Into::into).collect(),
					action.ability,
					action.target.map(Into::into),
					queue,
				)
			})
	}
}

impl ToActions for UnitsActions<Vec2> {
	fn to_iter(self) -> impl Iterator<Item = Action> {
		type ActionsOptUnit = HashMap<(Ability, Option<Tag>), HashSet<Tag>>;
		type ActionsPos = Vec<(Ability, Vec2, Tag)>;
		type Actions = (ActionsOptUnit, ActionsPos);
		let mut force_actions = Actions::default();
		let mut queue_actions = Vec::<Actions>::default();

		fn add(actions: &mut Actions, action: UnitAction<Vec2>, tag: Tag) {
			let target = match action.target {
				None => None,
				Some(Target::Unit(tag)) => Some(tag),
				Some(Target::Pos(pos)) => {
					actions.1.push((action.ability, pos, tag));
					return;
				}
			};
			let action = (action.ability, target);
			actions.0.entry(action).or_default().insert(tag);
		}

		for (tag, unit_actions) in self.actions {
			let mut it = unit_actions.actions.into_iter();

			if !unit_actions.queue {
				let Some(action) = it.next() else { continue };
				add(&mut force_actions, action, tag);
			}

			for (i, action) in (0..).zip(it) {
				if let Some(q_actions) = queue_actions.get_mut(i) {
					add(q_actions, action, tag);
				} else {
					let mut q_actions = Default::default();
					add(&mut q_actions, action, tag);
					queue_actions.push(q_actions);
				}
			}
		}

		fn to_actions(actions: Actions, queue: bool) -> impl Iterator<Item = Action> {
			actions
				.0
				.into_iter()
				.map(move |((ability, target), tags)| {
					(
						ability,
						target.map(Into::into).map(PbTarget::UnitTag),
						tags.into_iter().map(Into::into).collect(),
						queue,
					)
				})
				.chain(actions.1.into_iter().map(move |(ability, pos, tag)| {
					(
						ability,
						Some(PbTarget::WorldSpacePos(pos.into())),
						vec![tag.into()],
						queue,
					)
				}))
				.map(|(ability, target, tags, queue)| {
					raw_units_action(tags, ability, target, queue)
				})
		}

		to_actions(force_actions, false).chain(
			queue_actions
				.into_iter()
				.flat_map(|q_actions| to_actions(q_actions, true)),
		)
	}
}
