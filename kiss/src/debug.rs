use crate::linalg::{Vec2, Vec3};

use sc2_prost::{DebugCommand, debug_command::Command, debug_end_game::EndResult};
pub use sc2_prost::{DebugGameState as Cheat, debug_set_unit_value::UnitValue};

fn cmd(c: Command) -> DebugCommand {
	DebugCommand { command: Some(c) }
}

#[macro_export]
macro_rules! debug {
	() => {
		Vec::<$crate::debug::DebugCommand>::new()
	};
	($($item:expr),+ $(,)?) => {

		vec![$(Into::<$crate::debug::DebugCommand>::into($item)),+]
	};
}

pub fn cheat(cheat: Cheat) -> DebugCommand {
	cmd(Command::GameState(cheat as i32))
}

pub mod draw {
	use super::*;

	#[derive(Debug, Default, Clone, PartialEq)]
	pub struct Draw(DebugDraw);
	impl Draw {
		pub fn new() -> Self {
			Self::default()
		}
	}
	impl From<Draw> for DebugCommand {
		fn from(draw: Draw) -> Self {
			cmd(Command::Draw(draw.0))
		}
	}
	impl FromIterator<Item> for Draw {
		fn from_iter<I>(iter: I) -> Self
		where
			I: IntoIterator<Item = Item>,
		{
			let mut draw = Self(DebugDraw::default());
			draw.extend(iter);
			draw
		}
	}
	impl Extend<Item> for Draw {
		fn extend<I>(&mut self, iter: I)
		where
			I: IntoIterator<Item = Item>,
		{
			for item in iter {
				match item {
					Item::Text(text) => self.0.text.push(text.into()),
					Item::Line(line) => self.0.lines.push(line.into()),
					Item::AABB(aabb) => self.0.boxes.push(aabb.into()),
					Item::Sphere(sphere) => self.0.spheres.push(sphere.into()),
				}
			}
		}
	}

	#[derive(Debug, Clone, PartialEq)]
	pub enum Item {
		Text(Text),
		Line(Line),
		AABB(AABB),
		Sphere(Sphere),
	}
	impl From<Text> for Item {
		fn from(item: Text) -> Self {
			Self::Text(item)
		}
	}
	impl From<Line> for Item {
		fn from(item: Line) -> Self {
			Self::Line(item)
		}
	}
	impl From<AABB> for Item {
		fn from(item: AABB) -> Self {
			Self::AABB(item)
		}
	}
	impl From<Sphere> for Item {
		fn from(item: Sphere) -> Self {
			Self::Sphere(item)
		}
	}

	#[derive(Debug, Default, Clone, PartialEq)]
	pub struct Text {
		pub text: String,
		pub pos: TextPos,
		pub size: FontSize,
		pub color: Color,
	}
	#[macro_export]
	macro_rules! text {
		($($arg:tt)*) => {
			$crate::debug::draw::Text {
				text: format!($($arg)*),
				..Default::default()
			}
		};
	}
	use sc2_prost::DebugDraw;
	pub use text;
	impl Text {
		pub fn pos(mut self, pos: impl Into<TextPos>) -> Self {
			self.pos = pos.into();
			self
		}
		pub fn size(mut self, size: impl Into<FontSize>) -> Self {
			self.size = size.into();
			self
		}
		pub fn color(mut self, color: impl Into<Color>) -> Self {
			self.color = color.into();
			self
		}
	}
	impl From<Text> for sc2_prost::DebugText {
		fn from(t: Text) -> Self {
			let mut text = Self {
				color: t.color.into(),
				text: t.text,
				size: if t.size.0 == 8 { 0 } else { t.size.0 },
				..Default::default()
			};
			match t.pos {
				TextPos::Screen(p) => text.virtual_pos = Some(p.extend(0.).into()),
				TextPos::World(p) => text.world_pos = Some(p.into()),
			}
			text
		}
	}

	#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct FontSize(pub u32);
	impl From<u32> for FontSize {
		fn from(n: u32) -> Self {
			Self(n)
		}
	}

	#[derive(Debug, Clone, Copy, PartialEq)]
	pub enum TextPos {
		World(Vec3),
		Screen(Vec2),
	}
	impl Default for TextPos {
		fn default() -> Self {
			TextPos::Screen(Vec2::ZERO)
		}
	}
	impl From<Vec3> for TextPos {
		fn from(v: Vec3) -> Self {
			Self::World(v)
		}
	}
	impl From<Vec2> for TextPos {
		fn from(v: Vec2) -> Self {
			Self::Screen(v)
		}
	}
	impl From<(f32, f32)> for TextPos {
		fn from(t: (f32, f32)) -> Self {
			Self::Screen(t.into())
		}
	}

	#[derive(Debug, Default, Clone, Copy, PartialEq)]
	pub struct Line {
		pub p0: Vec3,
		pub p1: Vec3,
		pub color: Color,
	}
	pub fn line(p0: Vec3, p1: Vec3) -> Line {
		Line {
			p0,
			p1,
			..Default::default()
		}
	}
	impl Line {
		pub fn color(mut self, color: impl Into<Color>) -> Self {
			self.color = color.into();
			self
		}
	}
	impl From<Line> for sc2_prost::DebugLine {
		fn from(l: Line) -> Self {
			Self {
				color: l.color.into(),
				line: Some(sc2_prost::Line {
					p0: Some(l.p0.into()),
					p1: Some(l.p1.into()),
				}),
			}
		}
	}

	#[derive(Debug, Default, Clone, Copy, PartialEq)]
	pub struct AABB {
		pub p0: Vec3,
		pub p1: Vec3,
		pub color: Color,
	}
	pub fn aabb(p0: Vec3, p1: Vec3) -> AABB {
		AABB {
			p0,
			p1,
			..Default::default()
		}
	}
	pub fn cube(pos: Vec3, size: f32) -> AABB {
		let hs = size / 2.;
		AABB {
			p0: pos - hs,
			p1: pos + hs,
			..Default::default()
		}
	}
	pub fn square(pos: Vec3, size: f32) -> AABB {
		let z = pos.z;
		let pos2 = pos.truncate();
		let hs = size / 2.;
		AABB {
			p0: (pos2 - hs).extend(z),
			p1: (pos2 + hs).extend(z),
			..Default::default()
		}
	}
	impl AABB {
		pub fn color(mut self, color: impl Into<Color>) -> Self {
			self.color = color.into();
			self
		}
	}
	impl From<AABB> for sc2_prost::DebugBox {
		fn from(b: AABB) -> Self {
			Self {
				color: b.color.into(),
				min: Some(b.p0.into()),
				max: Some(b.p1.into()),
			}
		}
	}

	#[derive(Debug, Default, Clone, Copy, PartialEq)]
	pub struct Sphere {
		pub pos: Vec3,
		pub radius: f32,
		pub color: Color,
	}
	pub fn sphere(pos: Vec3, radius: f32) -> Sphere {
		Sphere {
			pos,
			radius,
			..Default::default()
		}
	}
	impl Sphere {
		pub fn radius(self, r: f32) -> Self {
			Self { radius: r, ..self }
		}
		pub fn size(self, d: f32) -> Self {
			self.radius(d / 2.)
		}
		pub fn color(mut self, color: impl Into<Color>) -> Self {
			self.color = color.into();
			self
		}
	}
	impl From<Sphere> for sc2_prost::DebugSphere {
		fn from(s: Sphere) -> Self {
			Self {
				color: s.color.into(),
				p: Some(s.pos.into()),
				r: s.radius,
			}
		}
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct Color {
		pub r: u8,
		pub g: u8,
		pub b: u8,
	}
	impl Color {
		pub const BLACK: Self = Self::new(0, 0, 0);
		pub const WHITE: Self = Self::new(255, 255, 255);
		pub const RED: Self = Self::new(255, 0, 0);
		pub const GREEN: Self = Self::new(0, 255, 0);
		pub const BLUE: Self = Self::new(0, 0, 255);
		pub const YELLOW: Self = Self::new(255, 255, 0);
		pub const PURPLE: Self = Self::new(255, 0, 255);
		pub const CYAN: Self = Self::new(0, 255, 255);

		pub const fn new(r: u8, g: u8, b: u8) -> Self {
			Self { r, g, b }
		}
	}
	impl Default for Color {
		fn default() -> Self {
			Self::WHITE
		}
	}
	impl From<(u8, u8, u8)> for Color {
		fn from(t: (u8, u8, u8)) -> Self {
			Self::new(t.0, t.1, t.2)
		}
	}
	impl From<Color> for sc2_prost::Color {
		fn from(c: Color) -> Self {
			// sc2 defaults to white color
			// so 255 doesn't need to be set explicitly
			fn cmap(c: u8) -> Option<u32> {
				(c != 255).then_some(c as u32)
			}
			Self {
				r: cmap(c.r),
				g: cmap(c.g),
				b: cmap(c.b),
			}
		}
	}
	impl From<Color> for Option<sc2_prost::Color> {
		fn from(c: Color) -> Self {
			(c != Color::WHITE).then(|| c.into())
		}
	}
}

pub mod unit {
	use super::*;
	use crate::{ids::UnitKind, unit::Tag};
	use sc2_core::common::PlayerId;

	#[derive(Debug, Default, Clone, Copy, PartialEq)]
	pub struct SpawnUnits {
		kind: UnitKind,
		count: u32,
		player: PlayerId,
		pos: Vec2,
	}
	pub fn spawn(kind: UnitKind) -> SpawnUnits {
		SpawnUnits {
			kind,
			..Default::default()
		}
	}
	impl SpawnUnits {
		pub fn count(self, count: u32) -> Self {
			Self { count, ..self }
		}
		pub fn player(self, player: PlayerId) -> Self {
			Self { player, ..self }
		}
		pub fn pos(self, pos: Vec2) -> Self {
			Self { pos, ..self }
		}
	}
	impl From<SpawnUnits> for DebugCommand {
		fn from(spawn_units: SpawnUnits) -> Self {
			cmd(Command::CreateUnit(sc2_prost::DebugCreateUnit {
				unit_type: spawn_units.kind.0,
				owner: spawn_units.player.0,
				pos: Some(spawn_units.pos.into()),
				quantity: spawn_units.count,
			}))
		}
	}

	pub fn kill(tags: impl IntoIterator<Item = Tag>) -> DebugCommand {
		let tag = tags.into_iter().map(Into::into).collect();
		cmd(Command::KillUnit(sc2_prost::DebugKillUnit { tag }))
	}

	#[derive(Debug, Default, Clone, Copy, PartialEq)]
	pub struct SetUnitValue {
		pub val: UnitValue,
		pub num: f32,
	}
	pub fn set_energy(num: f32) -> SetUnitValue {
		SetUnitValue {
			val: UnitValue::Energy,
			num,
		}
	}
	pub fn set_health(num: f32) -> SetUnitValue {
		SetUnitValue {
			val: UnitValue::Life,
			num,
		}
	}
	pub fn set_shield(num: f32) -> SetUnitValue {
		SetUnitValue {
			val: UnitValue::Shields,
			num,
		}
	}
	impl SetUnitValue {
		pub fn of(self, tag: Tag) -> DebugCommand {
			cmd(Command::UnitValue(sc2_prost::DebugSetUnitValue {
				unit_value: self.val as i32,
				value: self.num,
				unit_tag: tag.into(),
			}))
		}
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndGame(EndResult);
impl EndGame {
	pub const SURRENDER: Self = Self(EndResult::Unset);
	pub const VICTORY: Self = Self(EndResult::DeclareVictory);
}

impl From<EndGame> for DebugCommand {
	fn from(e: EndGame) -> Self {
		let e = e.0 as i32;
		cmd(Command::EndGame(sc2_prost::DebugEndGame { end_result: e }))
	}
}
