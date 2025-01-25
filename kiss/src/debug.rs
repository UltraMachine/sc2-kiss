use super::*;
use std::mem;

use sc2_core::{common::PlayerId, Client, Result};
use sc2_prost::{debug_command::Command as Cmd, DebugDraw};

pub use sc2_prost::{debug_set_unit_value::UnitValue, DebugGameState as Cheat};

use ids::UnitKind;
use linalg::{Vec2, Vec3};
use unit::Tag;

#[derive(Debug, Default)]
pub struct GameDebug {
	cmds: Vec<Cmd>,
	draw: Option<DebugDraw>,
	clear: bool,
}
impl GameDebug {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn flush(&mut self, client: &mut Client) -> Result {
		if let Some(draw) = self.draw.take() {
			self.cmds.push(Cmd::Draw(draw));
			self.clear = true;
		}
		if self.cmds.is_empty() {
			if self.clear {
				self.clear = false;
				return client.debug(vec![Cmd::Draw(<_>::default())]).map(|_| ());
			}
			return Ok(());
		}
		client.debug(mem::take(&mut self.cmds)).map(|_| ())
	}
	pub fn cheat(&mut self, c: Cheat) {
		self.cmds.push(Cmd::GameState(c as i32));
	}

	fn draw(&mut self) -> &mut DebugDraw {
		self.draw.get_or_insert_with(<_>::default)
	}
	pub fn draw_text(&mut self, text: String, pos: TextPos, font_size: FontSize, color: Color) {
		let mut item = sc2_prost::DebugText {
			color: Some(color.into()),
			text,
			size: if font_size.0 == 8 { 0 } else { font_size.0 },
			..<_>::default()
		};
		match pos {
			TextPos::Screen(p) => item.virtual_pos = Some(p.extend(0.).into()),
			TextPos::World(p) => item.world_pos = Some(p.into()),
		}
		self.draw().text.push(item);
	}
	pub fn draw_line(&mut self, p0: Vec3, p1: Vec3, color: Color) {
		let item = sc2_prost::DebugLine {
			color: Some(color.into()),
			line: Some(sc2_prost::Line {
				p0: Some(p0.into()),
				p1: Some(p1.into()),
			}),
		};
		self.draw().lines.push(item);
	}
	pub fn draw_box(&mut self, p0: Vec3, p1: Vec3, color: Color) {
		let item = sc2_prost::DebugBox {
			color: Some(color.into()),
			min: Some(p0.into()),
			max: Some(p1.into()),
		};
		self.draw().boxes.push(item);
	}
	pub fn draw_sphere(&mut self, pos: Vec3, rad: f32, color: Color) {
		let item = sc2_prost::DebugSphere {
			color: Some(color.into()),
			p: Some(pos.into()),
			r: rad,
		};
		self.draw().spheres.push(item);
	}

	pub fn spawn_units(&mut self, kind: UnitKind, count: u32, player: PlayerId, pos: Vec2) {
		let unit = sc2_prost::DebugCreateUnit {
			unit_type: kind.0,
			owner: player.into(),
			pos: Some(pos.into()),
			quantity: count,
		};
		self.cmds.push(Cmd::CreateUnit(unit));
	}
	pub fn kill_units(&mut self, tags: impl IntoIterator<Item = Tag>) {
		let tag = tags.into_iter().map(Into::into).collect();
		let item = sc2_prost::DebugKillUnit { tag };
		self.cmds.push(Cmd::KillUnit(item));
	}
	pub fn set_unit_value(&mut self, val: UnitValue, num: f32, tag: Tag) {
		let item = sc2_prost::DebugSetUnitValue {
			unit_value: val as i32,
			value: num,
			unit_tag: tag.into(),
		};
		self.cmds.push(Cmd::UnitValue(item))
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
impl From<Color> for sc2_prost::Color {
	fn from(c: Color) -> Self {
		// sc2 defaults to white color
		// so 0 is mapped to 255
		// bits higher than 8 seem to be dropped
		// so 256 is used to set 0
		fn cfix(c: u8) -> u32 {
			match c {
				0 => 256,
				255 => 0,
				_ => c as u32,
			}
		}
		Self {
			r: cfix(c.r),
			g: cfix(c.g),
			b: cfix(c.b),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontSize(pub u32);

impl Default for FontSize {
	fn default() -> Self {
		Self(8)
	}
}

#[derive(Debug)]
pub enum TextPos {
	World(Vec3),
	Screen(Vec2),
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
