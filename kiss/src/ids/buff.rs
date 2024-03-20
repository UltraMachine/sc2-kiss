use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Buff {
	#[default]
	None = 0,
	Radar25 = 1,
	Tauntb = 2,
	DisableAbils = 3,
	TransientMorph = 4,
	GravitonBeam = 5,
	GhostCloak = 6,
	BansheeCloak = 7,
	PowerUserWarpable = 8,
	VortexBehaviorEnemy = 9,
	Corruption = 10,
	QueenSpawnLarvaTimer = 11,
	GhostHoldFire = 12,
	GhostHoldFireB = 13,
	Leech = 14,
	LeechDisableAbilities = 15,
	EmpDecloak = 16,
	FungalGrowth = 17,
	GuardianShield = 18,
	SeekerMissileTimeout = 19,
	TimeWarpProduction = 20,
	Ethereal = 21,
	NeuralParasite = 22,
	NeuralParasiteWait = 23,
	StimpackMarauder = 24,
	SupplyDrop = 25,
	_250MmStrikeCannons = 26,
	Stimpack = 27,
	PsiStorm = 28,
	CloakFieldEffect = 29,
	Charging = 30,
	AiDangerBuff = 31,
	VortexBehavior = 32,
	Slow = 33,
	TemporalRiftUnit = 34,
	SheepBusy = 35,
	Contaminated = 36,
	TimeScaleConversionBehavior = 37,
	BlindingCloudStructure = 38,
	CollapsibleRockTowerConjoinedSearch = 39,
	CollapsibleRockTowerRampDiagonalConjoinedSearch = 40,
	CollapsibleTerranTowerConjoinedSearch = 41,
	CollapsibleTerranTowerRampDiagonalConjoinedSearch = 42,
	DigesterCreepSprayVision = 43,
	InvulnerabilityShield = 44,
	MineDroneCountdown = 45,
	MothershipStasis = 46,
	MothershipStasisCaster = 47,
	MothershipCoreEnergizeVisual = 48,
	OracleRevelation = 49,
	GhostSnipeDoT = 50,
	NexusPhaseShift = 51,
	NexusInvulnerability = 52,
	RoughTerrainSearch = 53,
	RoughTerrainSlow = 54,
	OracleCloakField = 55,
	OracleCloakFieldEffect = 56,
	ScryerFriendly = 57,
	SpectreShield = 58,
	ViperConsumeStructure = 59,
	RestoreShields = 60,
	MercenaryCycloneMissiles = 61,
	MercenarySensorDish = 62,
	MercenaryShield = 63,
	Scryer = 64,
	StunRoundInitialBehavior = 65,
	BuildingShield = 66,
	LaserSight = 67,
	ProtectiveBarrier = 68,
	CorruptorGroundAttackDebuff = 69,
	BattlecruiserAntiAirDisable = 70,
	BuildingStasis = 71,
	Stasis = 72,
	ResourceStun = 73,
	MaximumThrust = 74,
	ChargeUp = 75,
	CloakUnit = 76,
	NullField = 77,
	Rescue = 78,
	Benign = 79,
	LaserTargeting = 80,
	Engage = 81,
	CapResource = 82,
	BlindingCloud = 83,
	DoomDamageDelay = 84,
	EyeStalk = 85,
	BurrowCharge = 86,
	Hidden = 87,
	MineDroneDot = 88,
	MedivacSpeedBoost = 89,
	ExtendBridgeExtendingBridgeNeWide8Out = 90,
	ExtendBridgeExtendingBridgeNwWide8Out = 91,
	ExtendBridgeExtendingBridgeNeWide10Out = 92,
	ExtendBridgeExtendingBridgeNwWide10Out = 93,
	ExtendBridgeExtendingBridgeNeWide12Out = 94,
	ExtendBridgeExtendingBridgeNwWide12Out = 95,
	PhaseShield = 96,
	Purify = 97,
	VoidSiphon = 98,
	OracleWeapon = 99,
	AntiAirWeaponSwitchCooldown = 100,
	ArbiterMpStasisField = 101,
	ImmortalOverload = 102,
	CloakingFieldTargeted = 103,
	LightningBomb = 104,
	OraclePhaseShift = 105,
	ReleaseInterceptorsCooldown = 106,
	ReleaseInterceptorsTimedLifeWarning = 107,
	ReleaseInterceptorsWanderDelay = 108,
	ReleaseInterceptorsBeacon = 109,
	ArbiterMpCloakFieldEffect = 110,
	PurificationNova = 111,
	CorruptionBombDamage = 112,
	CorsairMpDisruptionWeb = 113,
	DisruptorPush = 114,
	LightofAiur = 115,
	LockOn = 116,
	Overcharge = 117,
	OverchargeDamage = 118,
	OverchargeSpeedBoost = 119,
	SeekerMissile = 120,
	TemporalField = 121,
	VoidRaySwarmDamageBoost = 122,
	VoidMpImmortalReviveSupressed = 123,
	DevourerMpAcidSpores = 124,
	DefilerMpConsume = 125,
	DefilerMpDarkSwarm = 126,
	DefilerMpPlague = 127,
	QueenMpEnsnare = 128,
	OracleStasisTrapTarget = 129,
	SelfRepair = 130,
	AggressiveMutation = 131,
	ParasiticBomb = 132,
	ParasiticBombUnitKu = 133,
	ParasiticBombSecondaryUnitSearch = 134,
	AdeptDeathCheck = 135,
	LurkerHoldFire = 136,
	LurkerHoldFireB = 137,
	TimeStopStun = 138,
	SlaynElementalGrabStun = 139,
	PurificationNovaPost = 140,
	DisableInterceptors = 141,
	BypassArmorDebuffOne = 142,
	BypassArmorDebuffTwo = 143,
	BypassArmorDebuffThree = 144,
	ChannelSnipeCombat = 145,
	TempestDisruptionBlastStunBehavior = 146,
	GravitonPrison = 147,
	InfestorDisease = 148,
	CarryMineralFieldMinerals = 271,
	CarryHighYieldMineralFieldMinerals = 272,
	CarryHarvestableVespeneGeyserGas = 273,
	CarryHarvestableVespeneGeyserGasProtoss = 274,
	CarryHarvestableVespeneGeyserGasZerg = 275,
	PermanentlyCloaked = 276,
	RavenScramblerMissile = 277,
	RavenShredderMissileTimeout = 278,
	RavenShredderMissileTint = 279,
	RavenShredderMissileArmorReduction = 280,
	ChronoBoostEnergyCost = 281,
	NexusShieldRechargeOnPylonBehavior = 282,
	NexusShieldRechargeOnPylonBehaviorSecondaryOnTarget = 283,
	InfestorEnsnare = 284,
	InfestorEnsnareMakePrecursorReheightSource = 285,
	NexusShieldOvercharge = 286,
	ParasiticBombDelayTimedLife = 287,
	Transfusion = 288,
	AccelerationZoneTemporalField = 289,
	AccelerationZoneFlyingTemporalField = 290,
	InhibitorZoneFlyingTemporalField = 291,
	InhibitorZoneTemporalField = 293,
	ResonatingGlaivesPhaseShift = 294,
	NeuralParasiteChildren = 295,
	AmorphousArmorcloud = 296,
	RavenShredderMissileArmorReductionUiSubtruct = 297,
	BatteryOvercharge = 298,
	OnCreepQueen = 304,
	CloakField = 305,
	TakenDamage = 306,
	RavenScramblerMissileCarrier = 307,
}
