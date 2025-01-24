use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Buff(pub u32);
impl Buff {
	pub const NONE: Self = Self(0);
	pub const RADAR_25: Self = Self(1);
	pub const TAUNTB: Self = Self(2);
	pub const DISABLE_ABILS: Self = Self(3);
	pub const TRANSIENT_MORPH: Self = Self(4);
	pub const GRAVITON_BEAM: Self = Self(5);
	pub const GHOST_CLOAK: Self = Self(6);
	pub const BANSHEE_CLOAK: Self = Self(7);
	pub const POWER_USER_WARPABLE: Self = Self(8);
	pub const VORTEX_BEHAVIOR_ENEMY: Self = Self(9);
	pub const CORRUPTION: Self = Self(10);
	pub const QUEEN_SPAWN_LARVA_TIMER: Self = Self(11);
	pub const GHOST_HOLD_FIRE: Self = Self(12);
	pub const GHOST_HOLD_FIRE_B: Self = Self(13);
	pub const LEECH: Self = Self(14);
	pub const LEECH_DISABLE_ABILITIES: Self = Self(15);
	pub const EMP_DECLOAK: Self = Self(16);
	pub const FUNGAL_GROWTH: Self = Self(17);
	pub const GUARDIAN_SHIELD: Self = Self(18);
	pub const SEEKER_MISSILE_TIMEOUT: Self = Self(19);
	pub const TIME_WARP_PRODUCTION: Self = Self(20);
	pub const ETHEREAL: Self = Self(21);
	pub const NEURAL_PARASITE: Self = Self(22);
	pub const NEURAL_PARASITE_WAIT: Self = Self(23);
	pub const STIMPACK_MARAUDER: Self = Self(24);
	pub const SUPPLY_DROP: Self = Self(25);
	pub const STIMPACK: Self = Self(27);
	pub const PSI_STORM: Self = Self(28);
	pub const CLOAK_FIELD_EFFECT: Self = Self(29);
	pub const CHARGING: Self = Self(30);
	pub const AI_DANGER_BUFF: Self = Self(31);
	pub const VORTEX_BEHAVIOR: Self = Self(32);
	pub const SLOW: Self = Self(33);
	pub const TEMPORAL_RIFT_UNIT: Self = Self(34);
	pub const SHEEP_BUSY: Self = Self(35);
	pub const CONTAMINATED: Self = Self(36);
	pub const TIME_SCALE_CONVERSION_BEHAVIOR: Self = Self(37);
	pub const BLINDING_CLOUD_STRUCTURE: Self = Self(38);
	pub const COLLAPSIBLE_ROCK_TOWER_CONJOINED_SEARCH: Self = Self(39);
	pub const COLLAPSIBLE_ROCK_TOWER_RAMP_DIAGONAL_CONJOINED_SEARCH: Self = Self(40);
	pub const COLLAPSIBLE_TERRAN_TOWER_CONJOINED_SEARCH: Self = Self(41);
	pub const COLLAPSIBLE_TERRAN_TOWER_RAMP_DIAGONAL_CONJOINED_SEARCH: Self = Self(42);
	pub const DIGESTER_CREEP_SPRAY_VISION: Self = Self(43);
	pub const INVULNERABILITY_SHIELD: Self = Self(44);
	pub const MINE_DRONE_COUNTDOWN: Self = Self(45);
	pub const MOTHERSHIP_STASIS: Self = Self(46);
	pub const MOTHERSHIP_STASIS_CASTER: Self = Self(47);
	pub const MOTHERSHIP_CORE_ENERGIZE_VISUAL: Self = Self(48);
	pub const ORACLE_REVELATION: Self = Self(49);
	pub const GHOST_SNIPE_DO_T: Self = Self(50);
	pub const NEXUS_PHASE_SHIFT: Self = Self(51);
	pub const NEXUS_INVULNERABILITY: Self = Self(52);
	pub const ROUGH_TERRAIN_SEARCH: Self = Self(53);
	pub const ROUGH_TERRAIN_SLOW: Self = Self(54);
	pub const ORACLE_CLOAK_FIELD: Self = Self(55);
	pub const ORACLE_CLOAK_FIELD_EFFECT: Self = Self(56);
	pub const SCRYER_FRIENDLY: Self = Self(57);
	pub const SPECTRE_SHIELD: Self = Self(58);
	pub const VIPER_CONSUME_STRUCTURE: Self = Self(59);
	pub const RESTORE_SHIELDS: Self = Self(60);
	pub const MERCENARY_CYCLONE_MISSILES: Self = Self(61);
	pub const MERCENARY_SENSOR_DISH: Self = Self(62);
	pub const MERCENARY_SHIELD: Self = Self(63);
	pub const SCRYER: Self = Self(64);
	pub const STUN_ROUND_INITIAL_BEHAVIOR: Self = Self(65);
	pub const BUILDING_SHIELD: Self = Self(66);
	pub const LASER_SIGHT: Self = Self(67);
	pub const PROTECTIVE_BARRIER: Self = Self(68);
	pub const CORRUPTOR_GROUND_ATTACK_DEBUFF: Self = Self(69);
	pub const BATTLECRUISER_ANTI_AIR_DISABLE: Self = Self(70);
	pub const BUILDING_STASIS: Self = Self(71);
	pub const STASIS: Self = Self(72);
	pub const RESOURCE_STUN: Self = Self(73);
	pub const MAXIMUM_THRUST: Self = Self(74);
	pub const CHARGE_UP: Self = Self(75);
	pub const CLOAK_UNIT: Self = Self(76);
	pub const NULL_FIELD: Self = Self(77);
	pub const RESCUE: Self = Self(78);
	pub const BENIGN: Self = Self(79);
	pub const LASER_TARGETING: Self = Self(80);
	pub const ENGAGE: Self = Self(81);
	pub const CAP_RESOURCE: Self = Self(82);
	pub const BLINDING_CLOUD: Self = Self(83);
	pub const DOOM_DAMAGE_DELAY: Self = Self(84);
	pub const EYE_STALK: Self = Self(85);
	pub const BURROW_CHARGE: Self = Self(86);
	pub const HIDDEN: Self = Self(87);
	pub const MINE_DRONE_DOT: Self = Self(88);
	pub const MEDIVAC_SPEED_BOOST: Self = Self(89);
	pub const PHASE_SHIELD: Self = Self(96);
	pub const PURIFY: Self = Self(97);
	pub const VOID_SIPHON: Self = Self(98);
	pub const ORACLE_WEAPON: Self = Self(99);
	pub const ANTI_AIR_WEAPON_SWITCH_COOLDOWN: Self = Self(100);
	pub const ARBITER_MP_STASIS_FIELD: Self = Self(101);
	pub const IMMORTAL_OVERLOAD: Self = Self(102);
	pub const CLOAKING_FIELD_TARGETED: Self = Self(103);
	pub const LIGHTNING_BOMB: Self = Self(104);
	pub const ORACLE_PHASE_SHIFT: Self = Self(105);
	pub const RELEASE_INTERCEPTORS_COOLDOWN: Self = Self(106);
	pub const RELEASE_INTERCEPTORS_TIMED_LIFE_WARNING: Self = Self(107);
	pub const RELEASE_INTERCEPTORS_WANDER_DELAY: Self = Self(108);
	pub const RELEASE_INTERCEPTORS_BEACON: Self = Self(109);
	pub const ARBITER_MP_CLOAK_FIELD_EFFECT: Self = Self(110);
	pub const PURIFICATION_NOVA: Self = Self(111);
	pub const CORRUPTION_BOMB_DAMAGE: Self = Self(112);
	pub const CORSAIR_MP_DISRUPTION_WEB: Self = Self(113);
	pub const DISRUPTOR_PUSH: Self = Self(114);
	pub const LIGHTOF_AIUR: Self = Self(115);
	pub const LOCK_ON: Self = Self(116);
	pub const OVERCHARGE: Self = Self(117);
	pub const OVERCHARGE_DAMAGE: Self = Self(118);
	pub const OVERCHARGE_SPEED_BOOST: Self = Self(119);
	pub const SEEKER_MISSILE: Self = Self(120);
	pub const TEMPORAL_FIELD: Self = Self(121);
	pub const VOID_RAY_SWARM_DAMAGE_BOOST: Self = Self(122);
	pub const VOID_MP_IMMORTAL_REVIVE_SUPRESSED: Self = Self(123);
	pub const DEVOURER_MP_ACID_SPORES: Self = Self(124);
	pub const DEFILER_MP_CONSUME: Self = Self(125);
	pub const DEFILER_MP_DARK_SWARM: Self = Self(126);
	pub const DEFILER_MP_PLAGUE: Self = Self(127);
	pub const QUEEN_MP_ENSNARE: Self = Self(128);
	pub const ORACLE_STASIS_TRAP_TARGET: Self = Self(129);
	pub const SELF_REPAIR: Self = Self(130);
	pub const AGGRESSIVE_MUTATION: Self = Self(131);
	pub const PARASITIC_BOMB: Self = Self(132);
	pub const PARASITIC_BOMB_UNIT_KU: Self = Self(133);
	pub const PARASITIC_BOMB_SECONDARY_UNIT_SEARCH: Self = Self(134);
	pub const ADEPT_DEATH_CHECK: Self = Self(135);
	pub const LURKER_HOLD_FIRE: Self = Self(136);
	pub const LURKER_HOLD_FIRE_B: Self = Self(137);
	pub const TIME_STOP_STUN: Self = Self(138);
	pub const SLAYN_ELEMENTAL_GRAB_STUN: Self = Self(139);
	pub const PURIFICATION_NOVA_POST: Self = Self(140);
	pub const DISABLE_INTERCEPTORS: Self = Self(141);
	pub const BYPASS_ARMOR_DEBUFF_ONE: Self = Self(142);
	pub const BYPASS_ARMOR_DEBUFF_TWO: Self = Self(143);
	pub const BYPASS_ARMOR_DEBUFF_THREE: Self = Self(144);
	pub const CHANNEL_SNIPE_COMBAT: Self = Self(145);
	pub const TEMPEST_DISRUPTION_BLAST_STUN_BEHAVIOR: Self = Self(146);
	pub const GRAVITON_PRISON: Self = Self(147);
	pub const INFESTOR_DISEASE: Self = Self(148);
	pub const CARRY_MINERAL_FIELD_MINERALS: Self = Self(271);
	pub const CARRY_HIGH_YIELD_MINERAL_FIELD_MINERALS: Self = Self(272);
	pub const CARRY_HARVESTABLE_VESPENE_GEYSER_GAS: Self = Self(273);
	pub const CARRY_HARVESTABLE_VESPENE_GEYSER_GAS_PROTOSS: Self = Self(274);
	pub const CARRY_HARVESTABLE_VESPENE_GEYSER_GAS_ZERG: Self = Self(275);
	pub const PERMANENTLY_CLOAKED: Self = Self(276);
	pub const RAVEN_SCRAMBLER_MISSILE: Self = Self(277);
	pub const RAVEN_SHREDDER_MISSILE_TIMEOUT: Self = Self(278);
	pub const RAVEN_SHREDDER_MISSILE_TINT: Self = Self(279);
	pub const RAVEN_SHREDDER_MISSILE_ARMOR_REDUCTION: Self = Self(280);
	pub const CHRONO_BOOST_ENERGY_COST: Self = Self(281);
	pub const NEXUS_SHIELD_RECHARGE_ON_PYLON_BEHAVIOR: Self = Self(282);
	pub const NEXUS_SHIELD_RECHARGE_ON_PYLON_BEHAVIOR_SECONDARY_ON_TARGET: Self = Self(283);
	pub const INFESTOR_ENSNARE: Self = Self(284);
	pub const INFESTOR_ENSNARE_MAKE_PRECURSOR_REHEIGHT_SOURCE: Self = Self(285);
	pub const NEXUS_SHIELD_OVERCHARGE: Self = Self(286);
	pub const PARASITIC_BOMB_DELAY_TIMED_LIFE: Self = Self(287);
	pub const TRANSFUSION: Self = Self(288);
	pub const ACCELERATION_ZONE_TEMPORAL_FIELD: Self = Self(289);
	pub const ACCELERATION_ZONE_FLYING_TEMPORAL_FIELD: Self = Self(290);
	pub const INHIBITOR_ZONE_FLYING_TEMPORAL_FIELD: Self = Self(291);
	pub const INHIBITOR_ZONE_TEMPORAL_FIELD: Self = Self(293);
	pub const CLOAK_FIELD: Self = Self(294);
	pub const RESONATING_GLAIVES_PHASE_SHIFT: Self = Self(295);
	pub const NEURAL_PARASITE_CHILDREN: Self = Self(296);
	pub const AMORPHOUS_ARMORCLOUD: Self = Self(297);
	pub const RAVEN_SHREDDER_MISSILE_ARMOR_REDUCTION_UI_SUBTRUCT: Self = Self(298);
	pub const TAKEN_DAMAGE: Self = Self(299);
	pub const RAVEN_SCRAMBLER_MISSILE_CARRIER: Self = Self(300);
	pub const BATTERY_OVERCHARGE: Self = Self(301);
	pub const LOAD_OUT_SPRAY_TRACKER: Self = Self(302);
	pub const HYDRALISK_FRENZY: Self = Self(303);
}
impl fmt::Display for Buff {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match self.0 {
			0 => "None",
			1 => "Radar 25",
			2 => "Tauntb",
			3 => "Disable Abils",
			4 => "Transient Morph",
			5 => "Graviton Beam",
			6 => "Ghost Cloak",
			7 => "Banshee Cloak",
			8 => "Power User Warpable",
			9 => "Vortex Behavior Enemy",
			10 => "Corruption",
			11 => "Queen Spawn Larva Timer",
			12 => "Ghost Hold Fire",
			13 => "Ghost Hold Fire B",
			14 => "Leech",
			15 => "Leech Disable Abilities",
			16 => "Emp Decloak",
			17 => "Fungal Growth",
			18 => "Guardian Shield",
			19 => "Seeker Missile Timeout",
			20 => "Time Warp Production",
			21 => "Ethereal",
			22 => "Neural Parasite",
			23 => "Neural Parasite Wait",
			24 => "Stimpack Marauder",
			25 => "Supply Drop",
			27 => "Stimpack",
			28 => "Psi Storm",
			29 => "Cloak Field Effect",
			30 => "Charging",
			31 => "Ai Danger Buff",
			32 => "Vortex Behavior",
			33 => "Slow",
			34 => "Temporal Rift Unit",
			35 => "Sheep Busy",
			36 => "Contaminated",
			37 => "Time Scale Conversion Behavior",
			38 => "Blinding Cloud Structure",
			39 => "Collapsible Rock Tower Conjoined Search",
			40 => "Collapsible Rock Tower Ramp Diagonal Conjoined Search",
			41 => "Collapsible Terran Tower Conjoined Search",
			42 => "Collapsible Terran Tower Ramp Diagonal Conjoined Search",
			43 => "Digester Creep Spray Vision",
			44 => "Invulnerability Shield",
			45 => "Mine Drone Countdown",
			46 => "Mothership Stasis",
			47 => "Mothership Stasis Caster",
			48 => "Mothership Core Energize Visual",
			49 => "Oracle Revelation",
			50 => "Ghost Snipe Do T",
			51 => "Nexus Phase Shift",
			52 => "Nexus Invulnerability",
			53 => "Rough Terrain Search",
			54 => "Rough Terrain Slow",
			55 => "Oracle Cloak Field",
			56 => "Oracle Cloak Field Effect",
			57 => "Scryer Friendly",
			58 => "Spectre Shield",
			59 => "Viper Consume Structure",
			60 => "Restore Shields",
			61 => "Mercenary Cyclone Missiles",
			62 => "Mercenary Sensor Dish",
			63 => "Mercenary Shield",
			64 => "Scryer",
			65 => "Stun Round Initial Behavior",
			66 => "Building Shield",
			67 => "Laser Sight",
			68 => "Protective Barrier",
			69 => "Corruptor Ground Attack Debuff",
			70 => "Battlecruiser Anti Air Disable",
			71 => "Building Stasis",
			72 => "Stasis",
			73 => "Resource Stun",
			74 => "Maximum Thrust",
			75 => "Charge Up",
			76 => "Cloak Unit",
			77 => "Null Field",
			78 => "Rescue",
			79 => "Benign",
			80 => "Laser Targeting",
			81 => "Engage",
			82 => "Cap Resource",
			83 => "Blinding Cloud",
			84 => "Doom Damage Delay",
			85 => "Eye Stalk",
			86 => "Burrow Charge",
			87 => "Hidden",
			88 => "Mine Drone Dot",
			89 => "Medivac Speed Boost",
			96 => "Phase Shield",
			97 => "Purify",
			98 => "Void Siphon",
			99 => "Oracle Weapon",
			100 => "Anti Air Weapon Switch Cooldown",
			101 => "Arbiter Mp Stasis Field",
			102 => "Immortal Overload",
			103 => "Cloaking Field Targeted",
			104 => "Lightning Bomb",
			105 => "Oracle Phase Shift",
			106 => "Release Interceptors Cooldown",
			107 => "Release Interceptors Timed Life Warning",
			108 => "Release Interceptors Wander Delay",
			109 => "Release Interceptors Beacon",
			110 => "Arbiter Mp Cloak Field Effect",
			111 => "Purification Nova",
			112 => "Corruption Bomb Damage",
			113 => "Corsair Mp Disruption Web",
			114 => "Disruptor Push",
			115 => "Lightof Aiur",
			116 => "Lock On",
			117 => "Overcharge",
			118 => "Overcharge Damage",
			119 => "Overcharge Speed Boost",
			120 => "Seeker Missile",
			121 => "Temporal Field",
			122 => "Void Ray Swarm Damage Boost",
			123 => "Void Mp Immortal Revive Supressed",
			124 => "Devourer Mp Acid Spores",
			125 => "Defiler Mp Consume",
			126 => "Defiler Mp Dark Swarm",
			127 => "Defiler Mp Plague",
			128 => "Queen Mp Ensnare",
			129 => "Oracle Stasis Trap Target",
			130 => "Self Repair",
			131 => "Aggressive Mutation",
			132 => "Parasitic Bomb",
			133 => "Parasitic Bomb Unit Ku",
			134 => "Parasitic Bomb Secondary Unit Search",
			135 => "Adept Death Check",
			136 => "Lurker Hold Fire",
			137 => "Lurker Hold Fire B",
			138 => "Time Stop Stun",
			139 => "Slayn Elemental Grab Stun",
			140 => "Purification Nova Post",
			141 => "Disable Interceptors",
			142 => "Bypass Armor Debuff One",
			143 => "Bypass Armor Debuff Two",
			144 => "Bypass Armor Debuff Three",
			145 => "Channel Snipe Combat",
			146 => "Tempest Disruption Blast Stun Behavior",
			147 => "Graviton Prison",
			148 => "Infestor Disease",
			271 => "Carry Mineral Field Minerals",
			272 => "Carry High Yield Mineral Field Minerals",
			273 => "Carry Harvestable Vespene Geyser Gas",
			274 => "Carry Harvestable Vespene Geyser Gas Protoss",
			275 => "Carry Harvestable Vespene Geyser Gas Zerg",
			276 => "Permanently Cloaked",
			277 => "Raven Scrambler Missile",
			278 => "Raven Shredder Missile Timeout",
			279 => "Raven Shredder Missile Tint",
			280 => "Raven Shredder Missile Armor Reduction",
			281 => "Chrono Boost Energy Cost",
			282 => "Nexus Shield Recharge On Pylon Behavior",
			283 => "Nexus Shield Recharge On Pylon Behavior Secondary On Target",
			284 => "Infestor Ensnare",
			285 => "Infestor Ensnare Make Precursor Reheight Source",
			286 => "Nexus Shield Overcharge",
			287 => "Parasitic Bomb Delay Timed Life",
			288 => "Transfusion",
			289 => "Acceleration Zone Temporal Field",
			290 => "Acceleration Zone Flying Temporal Field",
			291 => "Inhibitor Zone Flying Temporal Field",
			293 => "Inhibitor Zone Temporal Field",
			294 => "Cloak Field",
			295 => "Resonating Glaives Phase Shift",
			296 => "Neural Parasite Children",
			297 => "Amorphous Armorcloud",
			298 => "Raven Shredder Missile Armor Reduction Ui Subtruct",
			299 => "Taken Damage",
			300 => "Raven Scrambler Missile Carrier",
			301 => "Battery Overcharge",
			302 => "Load Out Spray Tracker",
			303 => "Hydralisk Frenzy",
			_ => return write!(f, "{self:?}"),
		};
		write!(f, "{s}")
	}
}
impl From<u32> for Buff {
	fn from(n: u32) -> Self {
		Self(n)
	}
}
impl From<Buff> for u32 {
	fn from(id: Buff) -> Self {
		id.0
	}
}
