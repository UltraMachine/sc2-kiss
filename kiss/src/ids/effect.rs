use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Effect {
	#[default]
	None = 0,
	PsiStorm = 1,
	GuardianShield = 2,
	TemporalFieldGrowing = 3,
	TemporalField = 4,
	ThermalLance = 5,
	ScannerSweep = 6,
	NukeDot = 7,
	LiberatorDefenderZoneSetup = 8,
	LiberatorDefenderZone = 9,
	BlindingCloud = 10,
	CorrosiveBile = 11,
	LurkerSpines = 12,
}
