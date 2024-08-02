use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Effect(pub u32);
impl Effect {
	pub const NONE: Self = Self(0);
	pub const PSI_STORM: Self = Self(1);
	pub const GUARDIAN_SHIELD: Self = Self(2);
	pub const TEMPORAL_FIELD_GROWING: Self = Self(3);
	pub const TEMPORAL_FIELD: Self = Self(4);
	pub const THERMAL_LANCE: Self = Self(5);
	pub const SCANNER_SWEEP: Self = Self(6);
	pub const NUKE_DOT: Self = Self(7);
	pub const LIBERATOR_DEFENDER_ZONE_SETUP: Self = Self(8);
	pub const LIBERATOR_DEFENDER_ZONE: Self = Self(9);
	pub const BLINDING_CLOUD: Self = Self(10);
	pub const CORROSIVE_BILE: Self = Self(11);
	pub const LURKER_SPINES: Self = Self(12);
}
impl fmt::Display for Effect {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match self.0 {
			0 => "None",
			1 => "Psi Storm",
			2 => "Guardian Shield",
			3 => "Temporal Field Growing",
			4 => "Temporal Field",
			5 => "Thermal Lance",
			6 => "Scanner Sweep",
			7 => "Nuke Dot",
			8 => "Liberator Defender Zone Setup",
			9 => "Liberator Defender Zone",
			10 => "Blinding Cloud",
			11 => "Corrosive Bile",
			12 => "Lurker Spines",
			_ => return write!(f, "{self:?}"),
		};
		write!(f, "{s}")
	}
}
impl From<u32> for Effect {
	fn from(n: u32) -> Self {
		Self(n)
	}
}
impl From<Effect> for u32 {
	fn from(id: Effect) -> Self {
		id.0
	}
}
