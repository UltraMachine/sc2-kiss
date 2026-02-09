use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GameLoop(pub u32);
impl GameLoop {
	pub fn from_millis(millis: u32) -> Self {
		Self(millis * 14 / 625)
	}
	pub fn from_secs(secs: u32) -> Self {
		Self(secs * 112 / 5)
	}
	pub fn from_mins(mins: u32) -> Self {
		Self(mins * 1344)
	}
	pub fn from_hours(hours: u32) -> Self {
		Self(hours * 80640)
	}

	pub fn as_millis(self) -> u32 {
		self.0 * 625 / 14
	}
	pub fn as_secs(self) -> u32 {
		self.0 * 5 / 112
	}

	pub fn display_time(self) -> DisplayTime {
		self.into()
	}
}
impl From<u32> for GameLoop {
	fn from(n: u32) -> Self {
		Self(n)
	}
}
impl fmt::Display for GameLoop {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl From<GameLoop> for Duration {
	fn from(game_loop: GameLoop) -> Self {
		Self::from_millis(game_loop.as_millis() as u64)
	}
}

impl Add for GameLoop {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}
impl Sub for GameLoop {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0)
	}
}
impl Mul<u32> for GameLoop {
	type Output = Self;

	fn mul(self, rhs: u32) -> Self::Output {
		Self(self.0 * rhs)
	}
}
impl Div<u32> for GameLoop {
	type Output = Self;

	fn div(self, rhs: u32) -> Self::Output {
		Self(self.0 / rhs)
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayTime {
	pub hours: u8,
	pub mins: u8,
	pub secs: u8,
	pub millis: u16,
	pub always_show_hours: bool,
	pub show_millis: bool,
}
impl DisplayTime {
	pub fn always_show_hours(mut self, value: bool) -> Self {
		self.always_show_hours = value;
		self
	}
	pub fn show_millis(mut self, value: bool) -> Self {
		self.show_millis = value;
		self
	}
}
impl From<GameLoop> for DisplayTime {
	fn from(game_loop: GameLoop) -> Self {
		let total_millis = game_loop.as_millis();
		Self {
			hours: (total_millis / 3_600_000) as u8,
			mins: (total_millis / 60_000 % 60) as u8,
			secs: (total_millis / 1000 % 60) as u8,
			millis: (total_millis % 1000) as u16,
			..Default::default()
		}
	}
}
impl fmt::Display for DisplayTime {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.hours != 0 || self.always_show_hours {
			write!(f, "{}:", self.hours)?;
		}
		write!(f, "{:02}:{:02}", self.mins, self.secs)?;
		if self.show_millis {
			write!(f, ".{:03}", self.millis)?;
		}
		Ok(())
	}
}
