use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ability(pub u32);
impl Ability {
	pub const NONE: Self = Self(0);
	pub const SMART: Self = Self(1);
	pub const EFFECT_TAUNT: Self = Self(2);
	pub const MOVE_TURN: Self = Self(20);
	pub const ATTACK_ATTACK_TOWARDS: Self = Self(24);
	pub const ATTACK_ATTACK_BARRAGE: Self = Self(25);
	pub const MORPH_INFESTED_TERRANS: Self = Self(40);
	pub const EFFECT_EXPLODE: Self = Self(42);
	pub const RESEARCH_INTERCEPTOR_GRAVITON_CATAPULT: Self = Self(44);
	pub const RESEARCH_RESEARCH_INTERCEPTOR_LAUNCH_SPEED_UPGRADE: Self = Self(45);
	pub const RESEARCH_PHOENIX_ANION_PULSE_CRYSTALS: Self = Self(46);
	pub const RESEARCH_TEMPEST_RANGE_UPGRADE: Self = Self(47);
	pub const RESEARCH_RESEARCH_VOID_RAY_SPEED_UPGRADE: Self = Self(48);
	pub const RESEARCH_TEMPEST_RESEARCH_GROUND_ATTACK_UPGRADE: Self = Self(49);
	pub const EFFECT_FUNGAL_GROWTH: Self = Self(74);
	pub const EFFECT_GUARDIAN_SHIELD: Self = Self(76);
	pub const TRAIN_MOTHERSHIP: Self = Self(110);
	pub const EFFECT_FEEDBACK: Self = Self(140);
	pub const HALLUCINATION_ARCHON: Self = Self(146);
	pub const HALLUCINATION_COLOSSUS: Self = Self(148);
	pub const HALLUCINATION_HIGH_TEMPLAR: Self = Self(150);
	pub const HALLUCINATION_IMMORTAL: Self = Self(152);
	pub const HALLUCINATION_PHOENIX: Self = Self(154);
	pub const HALLUCINATION_PROBE: Self = Self(156);
	pub const HALLUCINATION_STALKER: Self = Self(158);
	pub const HALLUCINATION_VOID_RAY: Self = Self(160);
	pub const HALLUCINATION_WARP_PRISM: Self = Self(162);
	pub const HALLUCINATION_ZEALOT: Self = Self(164);
	pub const EFFECT_CALLDOWN_MULE: Self = Self(171);
	pub const EFFECT_GRAVITON_BEAM: Self = Self(173);
	pub const EFFECT_SPAWN_CHANGELING: Self = Self(181);
	pub const RESEARCH_GLIAL_REGENERATION: Self = Self(216);
	pub const RESEARCH_TUNNELING_CLAWS: Self = Self(217);
	pub const RESEARCH_ROACH_SUPPLY: Self = Self(218);
	pub const EFFECT_SAP_STRUCTURE: Self = Self(245);
	pub const EFFECT_NEURAL_PARASITE: Self = Self(249);
	pub const EFFECT_INJECT_LARVA: Self = Self(251);
	pub const EFFECT_SUPPLY_DROP: Self = Self(255);
	pub const RESEARCH_ANABOLIC_SYNTHESIS: Self = Self(263);
	pub const RESEARCH_CHITINOUS_PLATING: Self = Self(265);
	pub const ATTACK_ATTACK_WARP_PRISM: Self = Self(301);
	pub const ATTACK_WARP_PRISM_ATTACK_TOWARDS: Self = Self(302);
	pub const ATTACK_WARP_PRISM_ATTACK_BARRAGE: Self = Self(303);
	pub const BUILD_COMMAND_CENTER: Self = Self(318);
	pub const BUILD_SUPPLY_DEPOT: Self = Self(319);
	pub const BUILD_REFINERY: Self = Self(320);
	pub const BUILD_BARRACKS: Self = Self(321);
	pub const BUILD_ENGINEERING_BAY: Self = Self(322);
	pub const BUILD_MISSILE_TURRET: Self = Self(323);
	pub const BUILD_BUNKER: Self = Self(324);
	pub const TERRAN_BUILD_REFINERY: Self = Self(325);
	pub const BUILD_SENSOR_TOWER: Self = Self(326);
	pub const BUILD_GHOST_ACADEMY: Self = Self(327);
	pub const BUILD_FACTORY: Self = Self(328);
	pub const BUILD_STARPORT: Self = Self(329);
	pub const BUILD_ARMORY: Self = Self(331);
	pub const BUILD_FUSION_CORE: Self = Self(333);
	pub const EFFECT_HEAL: Self = Self(386);
	pub const MORPH_SIEGE_MODE: Self = Self(388);
	pub const MORPH_UNSIEGE: Self = Self(390);
	pub const EFFECT_SCAN: Self = Self(399);
	pub const EFFECT_YAMATO_GUN: Self = Self(401);
	pub const MORPH_VIKING_ASSAULT_MODE: Self = Self(403);
	pub const MORPH_VIKING_FIGHTER_MODE: Self = Self(405);
	pub const TRAIN_SCV: Self = Self(524);
	pub const MORPH_SUPPLY_DEPOT_LOWER: Self = Self(556);
	pub const MORPH_SUPPLY_DEPOT_RAISE: Self = Self(558);
	pub const TRAIN_MARINE: Self = Self(560);
	pub const TRAIN_REAPER: Self = Self(561);
	pub const TRAIN_GHOST: Self = Self(562);
	pub const TRAIN_MARAUDER: Self = Self(563);
	pub const TRAIN_SIEGE_TANK: Self = Self(591);
	pub const TRAIN_THOR: Self = Self(594);
	pub const TRAIN_HELLION: Self = Self(595);
	pub const TRAIN_HELLBAT: Self = Self(596);
	pub const TRAIN_CYCLONE: Self = Self(597);
	pub const TRAIN_WIDOW_MINE: Self = Self(614);
	pub const TRAIN_MEDIVAC: Self = Self(620);
	pub const TRAIN_BANSHEE: Self = Self(621);
	pub const TRAIN_RAVEN: Self = Self(622);
	pub const TRAIN_BATTLECRUISER: Self = Self(623);
	pub const TRAIN_VIKING_FIGHTER: Self = Self(624);
	pub const TRAIN_LIBERATOR: Self = Self(626);
	pub const RESEARCH_HI_SEC_AUTO_TRACKING: Self = Self(650);
	pub const RESEARCH_TERRAN_STRUCTURE_ARMOR_UPGRADE: Self = Self(651);
	pub const RESEARCH_NEOSTEEL_FRAME: Self = Self(655);
	pub const BUILD_NUKE: Self = Self(710);
	pub const RESEARCH_STIMPACK: Self = Self(730);
	pub const RESEARCH_COMBAT_SHIELD: Self = Self(731);
	pub const RESEARCH_CONCUSSIVE_SHELLS: Self = Self(732);
	pub const RESEARCH_INFERNAL_PREIGNITER: Self = Self(761);
	pub const RESEARCH_RESEARCH_TRANSFORMATION_SERVOS: Self = Self(763);
	pub const RESEARCH_DRILLING_CLAWS: Self = Self(764);
	pub const RESEARCH_RESEARCH_LOCK_ON_RANGE_UPGRADE: Self = Self(765);
	pub const RESEARCH_SMART_SERVOS: Self = Self(766);
	pub const RESEARCH_RESEARCH_ARMOR_PIERCING_ROCKETS: Self = Self(767);
	pub const RESEARCH_CYCLONE_RAPID_FIRE_LAUNCHERS: Self = Self(768);
	pub const RESEARCH_CYCLONE_LOCK_ON_DAMAGE: Self = Self(769);
	pub const RESEARCH_CYCLONE_RESEARCH_HURRICANE_THRUSTERS: Self = Self(770);
	pub const RESEARCH_BANSHEE_CLOAKING_FIELD: Self = Self(790);
	pub const RESEARCH_RESEARCH_MEDIVAC_ENERGY_UPGRADE: Self = Self(792);
	pub const RESEARCH_RAVEN_CORVID_REACTOR: Self = Self(793);
	pub const RESEARCH_RESEARCH_SEEKER_MISSILE: Self = Self(796);
	pub const RESEARCH_RESEARCH_DURABLE_MATERIALS: Self = Self(797);
	pub const RESEARCH_BANSHEE_HYPERFLIGHT_ROTORS: Self = Self(799);
	pub const RESEARCH_RESEARCH_LIBERATOR_AG_MODE: Self = Self(800);
	pub const RESEARCH_RESEARCH_RAPID_DEPLOYMENT: Self = Self(802);
	pub const RESEARCH_RAVEN_RECALIBRATED_EXPLOSIVES: Self = Self(803);
	pub const RESEARCH_HIGH_CAPACITY_FUEL_TANKS: Self = Self(804);
	pub const RESEARCH_ADVANCED_BALLISTICS: Self = Self(805);
	pub const RESEARCH_RAVEN_RESEARCH_ENHANCED_MUNITIONS: Self = Self(806);
	pub const RESEARCH_RESEARCH_RAVEN_INTERFERENCE_MATRIX: Self = Self(807);
	pub const RESEARCH_PERSONAL_CLOAKING: Self = Self(820);
	pub const RESEARCH_TERRAN_VEHICLE_PLATING_LEVEL_1: Self = Self(852);
	pub const RESEARCH_TERRAN_VEHICLE_PLATING_LEVEL_2: Self = Self(853);
	pub const RESEARCH_TERRAN_VEHICLE_PLATING_LEVEL_3: Self = Self(854);
	pub const RESEARCH_TERRAN_SHIP_PLATING_LEVEL_1: Self = Self(858);
	pub const RESEARCH_TERRAN_SHIP_PLATING_LEVEL_2: Self = Self(859);
	pub const RESEARCH_TERRAN_SHIP_PLATING_LEVEL_3: Self = Self(860);
	pub const BUILD_NEXUS: Self = Self(880);
	pub const BUILD_PYLON: Self = Self(881);
	pub const BUILD_ASSIMILATOR: Self = Self(882);
	pub const BUILD_GATEWAY: Self = Self(883);
	pub const BUILD_FORGE: Self = Self(884);
	pub const BUILD_FLEET_BEACON: Self = Self(885);
	pub const BUILD_TWILIGHT_COUNCIL: Self = Self(886);
	pub const BUILD_PHOTON_CANNON: Self = Self(887);
	pub const BUILD_STARGATE: Self = Self(889);
	pub const BUILD_TEMPLAR_ARCHIVE: Self = Self(890);
	pub const BUILD_DARK_SHRINE: Self = Self(891);
	pub const BUILD_ROBOTICS_BAY: Self = Self(892);
	pub const BUILD_ROBOTICS_FACILITY: Self = Self(893);
	pub const BUILD_CYBERNETICS_CORE: Self = Self(894);
	pub const BUILD_SHIELD_BATTERY: Self = Self(895);
	pub const TRAIN_ZEALOT: Self = Self(916);
	pub const TRAIN_STALKER: Self = Self(917);
	pub const TRAIN_HIGH_TEMPLAR: Self = Self(919);
	pub const TRAIN_DARK_TEMPLAR: Self = Self(920);
	pub const TRAIN_SENTRY: Self = Self(921);
	pub const TRAIN_ADEPT: Self = Self(922);
	pub const TRAIN_PHOENIX: Self = Self(946);
	pub const TRAIN_CARRIER: Self = Self(948);
	pub const TRAIN_VOID_RAY: Self = Self(950);
	pub const TRAIN_ORACLE: Self = Self(954);
	pub const TRAIN_TEMPEST: Self = Self(955);
	pub const TRAIN_WARP_PRISM: Self = Self(976);
	pub const TRAIN_OBSERVER: Self = Self(977);
	pub const TRAIN_COLOSSUS: Self = Self(978);
	pub const TRAIN_IMMORTAL: Self = Self(979);
	pub const TRAIN_DISRUPTOR: Self = Self(994);
	pub const TRAIN_PROBE: Self = Self(1006);
	pub const EFFECT_PSI_STORM: Self = Self(1036);
	pub const BUILD_INTERCEPTORS: Self = Self(1042);
	pub const RESEARCH_GRAVITIC_BOOSTER: Self = Self(1093);
	pub const RESEARCH_GRAVITIC_DRIVE: Self = Self(1094);
	pub const RESEARCH_EXTENDED_THERMAL_LANCE: Self = Self(1097);
	pub const RESEARCH_RESEARCH_IMMORTAL_REVIVE: Self = Self(1099);
	pub const RESEARCH_PSI_STORM: Self = Self(1126);
	pub const BUILD_HATCHERY: Self = Self(1152);
	pub const BUILD_CREEP_TUMOR: Self = Self(1153);
	pub const BUILD_EXTRACTOR: Self = Self(1154);
	pub const BUILD_SPAWNING_POOL: Self = Self(1155);
	pub const BUILD_EVOLUTION_CHAMBER: Self = Self(1156);
	pub const BUILD_HYDRALISK_DEN: Self = Self(1157);
	pub const BUILD_SPIRE: Self = Self(1158);
	pub const BUILD_ULTRALISK_CAVERN: Self = Self(1159);
	pub const BUILD_INFESTATION_PIT: Self = Self(1160);
	pub const BUILD_NYDUS_NETWORK: Self = Self(1161);
	pub const BUILD_BANELING_NEST: Self = Self(1162);
	pub const BUILD_LURKER_DEN: Self = Self(1163);
	pub const BUILD_ROACH_WARREN: Self = Self(1165);
	pub const BUILD_SPINE_CRAWLER: Self = Self(1166);
	pub const BUILD_SPORE_CRAWLER: Self = Self(1167);
	pub const RESEARCH_EVOLVE_PROPULSIVE_PERISTALSIS: Self = Self(1195);
	pub const MORPH_LAIR: Self = Self(1216);
	pub const MORPH_HIVE: Self = Self(1218);
	pub const MORPH_GREATER_SPIRE: Self = Self(1220);
	pub const RESEARCH_PNEUMATIZED_CARAPACE: Self = Self(1223);
	pub const RESEARCH_EVOLVE_VENTRAL_SACKS: Self = Self(1224);
	pub const RESEARCH_BURROW: Self = Self(1225);
	pub const RESEARCH_ZERGLING_ADRENAL_GLANDS: Self = Self(1252);
	pub const RESEARCH_ZERGLING_METABOLIC_BOOST: Self = Self(1253);
	pub const RESEARCH_GROOVED_SPINES: Self = Self(1282);
	pub const RESEARCH_MUSCULAR_AUGMENTS: Self = Self(1283);
	pub const RESEARCH_RESEARCH_FRENZY: Self = Self(1284);
	pub const RESEARCH_RESEARCH_LURKER_RANGE: Self = Self(1286);
	pub const TRAIN_DRONE: Self = Self(1342);
	pub const TRAIN_ZERGLING: Self = Self(1343);
	pub const TRAIN_OVERLORD: Self = Self(1344);
	pub const TRAIN_HYDRALISK: Self = Self(1345);
	pub const TRAIN_MUTALISK: Self = Self(1346);
	pub const TRAIN_ULTRALISK: Self = Self(1348);
	pub const TRAIN_ROACH: Self = Self(1351);
	pub const TRAIN_INFESTOR: Self = Self(1352);
	pub const TRAIN_CORRUPTOR: Self = Self(1353);
	pub const TRAIN_VIPER: Self = Self(1354);
	pub const TRAIN_SWARM_HOST: Self = Self(1356);
	pub const MORPH_BROOD_LORD: Self = Self(1372);
	pub const TRAIN_WARP_ZEALOT: Self = Self(1413);
	pub const TRAIN_WARP_STALKER: Self = Self(1414);
	pub const TRAIN_WARP_HIGH_TEMPLAR: Self = Self(1416);
	pub const TRAIN_WARP_DARK_TEMPLAR: Self = Self(1417);
	pub const TRAIN_WARP_SENTRY: Self = Self(1418);
	pub const TRAIN_WARP_ADEPT: Self = Self(1419);
	pub const MORPH_OVERSEER: Self = Self(1448);
	pub const MORPH_PLANETARY_FORTRESS: Self = Self(1450);
	pub const RESEARCH_NEURAL_PARASITE: Self = Self(1455);
	pub const RESEARCH_RESEARCH_LOCUST_LIFETIME_INCREASE: Self = Self(1456);
	pub const RESEARCH_EVOLVE_AMORPHOUS_ARMORCLOUD: Self = Self(1457);
	pub const RESEARCH_CENTRIFUGAL_HOOKS: Self = Self(1482);
	pub const MORPH_ORBITAL_COMMAND: Self = Self(1516);
	pub const MORPH_WARP_GATE: Self = Self(1518);
	pub const MORPH_GATEWAY: Self = Self(1520);
	pub const EFFECT_FORCE_FIELD: Self = Self(1526);
	pub const MORPH_WARP_PRISM_PHASING_MODE: Self = Self(1528);
	pub const MORPH_WARP_PRISM_TRANSPORT_MODE: Self = Self(1530);
	pub const RESEARCH_BATTLECRUISER_WEAPON_REFIT: Self = Self(1532);
	pub const RESEARCH_RESEARCH_BALLISTIC_RANGE: Self = Self(1533);
	pub const RESEARCH_RESEARCH_RAPID_REIGNITION_SYSTEM: Self = Self(1534);
	pub const FUSION_CORE_RESEARCH_RESEARCH_MEDIVAC_ENERGY_UPGRADE: Self = Self(1535);
	pub const RESEARCH_WARP_GATE: Self = Self(1568);
	pub const RESEARCH_RESEARCH_HALLUCINATION: Self = Self(1571);
	pub const RESEARCH_CHARGE: Self = Self(1592);
	pub const RESEARCH_BLINK: Self = Self(1593);
	pub const RESEARCH_ADEPT_RESONATING_GLAIVES: Self = Self(1594);
	pub const RESEARCH_RESEARCH_PSIONIC_SURGE: Self = Self(1595);
	pub const RESEARCH_RESEARCH_AMPLIFIED_SHIELDING: Self = Self(1596);
	pub const RESEARCH_RESEARCH_PSIONIC_AMPLIFIERS: Self = Self(1597);
	pub const EFFECT_NUKE_CALLDOWN: Self = Self(1622);
	pub const EFFECT_EMP: Self = Self(1628);
	pub const TRAIN_QUEEN: Self = Self(1632);
	pub const EFFECT_TRANSFUSION: Self = Self(1664);
	pub const MORPH_TECH_LAB_BARRACKS: Self = Self(1668);
	pub const MORPH_TECH_LAB_FACTORY: Self = Self(1670);
	pub const MORPH_TECH_LAB_STARPORT: Self = Self(1672);
	pub const MORPH_REACTOR: Self = Self(1676);
	pub const FACTORY_REACTOR_MORPH_REACTOR: Self = Self(1678);
	pub const STARPORT_REACTOR_MORPH_REACTOR: Self = Self(1680);
	pub const BEHAVIOR_GENERATE_CREEP_ON: Self = Self(1692);
	pub const BEHAVIOR_GENERATE_CREEP_OFF: Self = Self(1693);
	pub const EFFECT_AUTO_TURRET: Self = Self(1764);
	pub const MORPH_ARCHON: Self = Self(1766);
	pub const ARCHON_WARP_TARGET: Self = Self(1767);
	pub const BUILD_NYDUS_WORM: Self = Self(1768);
	pub const BUILD_SUMMON_NYDUS_CANAL_ATTACKER: Self = Self(1769);
	pub const EFFECT_CHARGE: Self = Self(1819);
	pub const EFFECT_HERD: Self = Self(1821);
	pub const EFFECT_CONTAMINATE: Self = Self(1825);
	pub const MORPH_MOVE: Self = Self(1837);
	pub const EFFECT_DIGESTER_CREEP_SPRAY: Self = Self(1839);
	pub const MORPH_MOTHERSHIP: Self = Self(1847);
	pub const EFFECT_XEL_NAGA_HEALING_SHRINE: Self = Self(1928);
	pub const EFFECT_NEXUS_INVULNERABILITY: Self = Self(1930);
	pub const MORPH_HELLION: Self = Self(1978);
	pub const MORPH_HELLBAT: Self = Self(1998);
	pub const ATTACK_PROTOSS_BUILDING_ATTACK_TOWARDS: Self = Self(2049);
	pub const ATTACK_PROTOSS_BUILDING_ATTACK_BARRAGE: Self = Self(2050);
	pub const STOP_HOLD_FIRE: Self = Self(2058);
	pub const STOP_CHEER: Self = Self(2059);
	pub const STOP_DANCE: Self = Self(2060);
	pub const EFFECT_BLINDING_CLOUD: Self = Self(2063);
	pub const EFFECT_ABDUCT: Self = Self(2067);
	pub const EFFECT_VIPER_CONSUME: Self = Self(2073);
	pub const BEHAVIOR_BUILDING_ATTACK_ON: Self = Self(2081);
	pub const BEHAVIOR_BUILDING_ATTACK_OFF: Self = Self(2082);
	pub const EFFECT_PICKUP_SCRAP_SMALL: Self = Self(2083);
	pub const EFFECT_PICKUP_SCRAP_MEDIUM: Self = Self(2085);
	pub const EFFECT_PICKUP_SCRAP_LARGE: Self = Self(2087);
	pub const EFFECT_PICKUP_PALLET_GAS: Self = Self(2089);
	pub const EFFECT_PICKUP_PALLET_MINERALS: Self = Self(2091);
	pub const EFFECT_MASSIVE_KNOCKOVER: Self = Self(2093);
	pub const EFFECT_WIDOW_MINE_ATTACK: Self = Self(2099);
	pub const AUGMENT_TORNADO_MISSILE: Self = Self(2101);
	pub const HALLUCINATION_ORACLE: Self = Self(2114);
	pub const EFFECT_MEDIVAC_IGNITE_AFTERBURNERS: Self = Self(2116);
	pub const EFFECT_CRITTER_FLEE: Self = Self(2144);
	pub const EFFECT_ORACLE_REVELATION: Self = Self(2146);
	pub const EFFECT_ULTRALISK_WEAPON_COOLDOWN: Self = Self(2158);
	pub const EFFECT_PHOTON_OVERCHARGE: Self = Self(2162);
	pub const EFFECT_TIME_WARP: Self = Self(2244);
	pub const EFFECT_CAUSTIC_SPRAY: Self = Self(2324);
	pub const MORPH_RAVAGER: Self = Self(2330);
	pub const MORPH_LURKER: Self = Self(2332);
	pub const EFFECT_CORROSIVE_BILE: Self = Self(2338);
	pub const EFFECT_PURIFICATION_NOVA: Self = Self(2344);
	pub const PURIFICATION_NOVA_TARGETED_PURIFICATION_NOVA_TARGETED: Self = Self(2346);
	pub const EFFECT_LOCK_ON: Self = Self(2350);
	pub const EFFECT_TACTICAL_JUMP: Self = Self(2358);
	pub const MORPH_THOR_HIGH_IMPACT_MODE: Self = Self(2362);
	pub const MORPH_THOR_EXPLOSIVE_MODE: Self = Self(2364);
	pub const BEHAVIOR_PULSAR_BEAM_ON: Self = Self(2375);
	pub const BEHAVIOR_PULSAR_BEAM_OFF: Self = Self(2376);
	pub const MORPH_LOCUST_MP_FLYING_SWOOP: Self = Self(2383);
	pub const LOCUST_MP_MORPH_TO_AIR_LOCUST_MP_FLYING_SWOOP: Self = Self(2385);
	pub const EFFECT_LOCUST_SWOOP: Self = Self(2387);
	pub const HALLUCINATION_DISRUPTOR: Self = Self(2389);
	pub const HALLUCINATION_ADEPT: Self = Self(2391);
	pub const EFFECT_VOID_RAY_PRISMATIC_ALIGNMENT: Self = Self(2393);
	pub const MORPH_IMMORTAL: Self = Self(2469);
	pub const EFFECT_ARBITER_MP_STASIS_FIELD: Self = Self(2473);
	pub const EFFECT_ARBITER_MP_RECALL: Self = Self(2475);
	pub const EFFECT_CORSAIR_MP_DISRUPTION_WEB: Self = Self(2477);
	pub const MORPH_MORPH_TO_GUARDIAN_MP: Self = Self(2479);
	pub const MORPH_MORPH_TO_DEVOURER_MP: Self = Self(2481);
	pub const EFFECT_DEFILER_MP_CONSUME: Self = Self(2483);
	pub const EFFECT_DEFILER_MP_DARK_SWARM: Self = Self(2485);
	pub const EFFECT_DEFILER_MP_PLAGUE: Self = Self(2487);
	pub const EFFECT_QUEEN_MP_ENSNARE: Self = Self(2493);
	pub const EFFECT_QUEEN_MP_SPAWN_BROODLINGS: Self = Self(2495);
	pub const EFFECT_QUEEN_MP_INFEST_COMMAND_CENTER: Self = Self(2497);
	pub const BUILD_STASIS_TRAP: Self = Self(2505);
	pub const EFFECT_ACTIVATE_STASIS_WARD: Self = Self(2536);
	pub const EFFECT_PARASITIC_BOMB: Self = Self(2542);
	pub const EFFECT_ADEPT_PHASE_SHIFT: Self = Self(2544);
	pub const MORPH_PURIFICATION_NOVA: Self = Self(2548);
	pub const MORPH_LIBERATOR_AG_MODE: Self = Self(2554);
	pub const MORPH_LIBERATOR_AA_MODE: Self = Self(2556);
	pub const LIBERATOR_AG_TARGET_LIBERATOR_AG_MODE: Self = Self(2558);
	pub const LIBERATOR_AA_TARGET_LIBERATOR_AA_MODE: Self = Self(2560);
	pub const EFFECT_KD_8_CHARGE: Self = Self(2588);
	pub const EFFECT_SLAYN_ELEMENTAL_GRAB: Self = Self(2598);
	pub const EFFECT_SPAWN_LOCUSTS: Self = Self(2704);
	pub const EFFECT_LOCUST_MP_FLYING_SWOOP: Self = Self(2706);
	pub const MORPH_OVERLORD_TRANSPORT: Self = Self(2708);
	pub const EFFECT_GHOST_SNIPE: Self = Self(2714);
	pub const MORPH_MOTHERSHIP_CORE_WEAPON: Self = Self(2716);
	pub const PURIFY_MORPH_PYLON_BACK_MOTHERSHIP_CORE_WEAPON: Self = Self(2718);
	pub const RESEARCH_SHADOW_STRIKE: Self = Self(2720);
	pub const CANCEL: Self = Self(3659);
	pub const HALT: Self = Self(3660);
	pub const BURROW_DOWN: Self = Self(3661);
	pub const BURROW_UP: Self = Self(3662);
	pub const LOAD_ALL: Self = Self(3663);
	pub const UNLOAD_ALL: Self = Self(3664);
	pub const STOP: Self = Self(3665);
	pub const HARVEST_GATHER: Self = Self(3666);
	pub const HARVEST_RETURN: Self = Self(3667);
	pub const LOAD: Self = Self(3668);
	pub const UNLOAD_ALL_AT: Self = Self(3669);
	pub const UNLOAD_UNIT: Self = Self(3670);
	pub const CANCEL_LAST: Self = Self(3671);
	pub const CANCEL_SLOT: Self = Self(3672);
	pub const RALLY_UNITS: Self = Self(3673);
	pub const ATTACK: Self = Self(3674);
	pub const EFFECT_STIM: Self = Self(3675);
	pub const BEHAVIOR_CLOAK_ON: Self = Self(3676);
	pub const BEHAVIOR_CLOAK_OFF: Self = Self(3677);
	pub const LAND: Self = Self(3678);
	pub const LIFT: Self = Self(3679);
	pub const MORPH_ROOT: Self = Self(3680);
	pub const MORPH_UPROOT: Self = Self(3681);
	pub const BUILD_TECH_LAB: Self = Self(3682);
	pub const BUILD_REACTOR: Self = Self(3683);
	pub const EFFECT_SPRAY: Self = Self(3684);
	pub const EFFECT_REPAIR: Self = Self(3685);
	pub const EFFECT_MASS_RECALL: Self = Self(3686);
	pub const EFFECT_BLINK: Self = Self(3687);
	pub const BEHAVIOR_HOLD_FIRE_ON: Self = Self(3688);
	pub const BEHAVIOR_HOLD_FIRE_OFF: Self = Self(3689);
	pub const RALLY_WORKERS: Self = Self(3690);
	pub const GENERAL_BUILD_CREEP_TUMOR_BUILD_CREEP_TUMOR: Self = Self(3691);
	pub const RESEARCH_PROTOSS_AIR_ARMOR: Self = Self(3692);
	pub const RESEARCH_PROTOSS_AIR_WEAPONS: Self = Self(3693);
	pub const RESEARCH_PROTOSS_GROUND_ARMOR: Self = Self(3694);
	pub const RESEARCH_PROTOSS_GROUND_WEAPONS: Self = Self(3695);
	pub const RESEARCH_PROTOSS_SHIELDS: Self = Self(3696);
	pub const RESEARCH_TERRAN_INFANTRY_ARMOR: Self = Self(3697);
	pub const RESEARCH_TERRAN_INFANTRY_WEAPONS: Self = Self(3698);
	pub const RESEARCH_TERRAN_SHIP_WEAPONS: Self = Self(3699);
	pub const RESEARCH_TERRAN_VEHICLE_AND_SHIP_PLATING: Self = Self(3700);
	pub const RESEARCH_TERRAN_VEHICLE_WEAPONS: Self = Self(3701);
	pub const RESEARCH_ZERG_FLYER_ARMOR: Self = Self(3702);
	pub const RESEARCH_ZERG_FLYER_ATTACK: Self = Self(3703);
	pub const RESEARCH_ZERG_GROUND_ARMOR: Self = Self(3704);
	pub const RESEARCH_ZERG_MELEE_WEAPONS: Self = Self(3705);
	pub const RESEARCH_ZERG_MISSILE_WEAPONS: Self = Self(3706);
	pub const RESEARCH_ADAPTIVE_TALONS: Self = Self(3709);
	pub const LURKER_DEN_RESEARCH_RESEARCH_LURKER_RANGE: Self = Self(3710);
	pub const MORPH_OBSERVER_MODE: Self = Self(3739);
	pub const MORPH_SURVEILLANCE_MODE: Self = Self(3741);
	pub const MORPH_OVERSIGHT_MODE: Self = Self(3743);
	pub const MORPH_OVERSEER_MODE: Self = Self(3745);
	pub const EFFECT_INTERFERENCE_MATRIX: Self = Self(3747);
	pub const EFFECT_ANTI_ARMOR_MISSILE: Self = Self(3753);
	pub const EFFECT_CHRONO_BOOST_ENERGY_COST: Self = Self(3755);
	pub const EFFECT_INFESTOR_ENSNARE: Self = Self(3763);
	pub const BATTLECRUISER_ATTACK_ATTACK_TOWARDS: Self = Self(3772);
	pub const BATTLECRUISER_ATTACK_ATTACK_BARRAGE: Self = Self(3773);
	pub const MOVE_ACQUIRE_MOVE: Self = Self(3779);
	pub const BATTLECRUISER_MOVE_TURN: Self = Self(3780);
	pub const BATTLECRUISER_STOP_HOLD_FIRE: Self = Self(3784);
	pub const BATTLECRUISER_STOP_CHEER: Self = Self(3785);
	pub const BATTLECRUISER_STOP_DANCE: Self = Self(3786);
	pub const VIPER_PARASITIC_BOMB_RELAY_PARASITIC_BOMB: Self = Self(3789);
	pub const PARASITIC_BOMB_RELAY_DODGE_PARASITIC_BOMB: Self = Self(3791);
	pub const HOLD_POSITION: Self = Self(3793);
	pub const MOVE: Self = Self(3794);
	pub const PATROL: Self = Self(3795);
	pub const GENERAL_UNLOAD_UNIT_UNLOAD_UNIT: Self = Self(3796);
	pub const EFFECT_BATTERY_OVERCHARGE: Self = Self(4107);
	pub const EFFECT_AMORPHOUS_ARMORCLOUD: Self = Self(4109);
	pub const EFFECT_SHIELD_BATTERY_RECHARGE: Self = Self(4111);
	pub const MORPH_BANELING: Self = Self(4119);
	pub const EFFECT_ORACLE_CLOAK_FIELD: Self = Self(4122);
	pub const EFFECT_ENERGY_RECHARGE: Self = Self(4440);
	pub const EFFECT_SALVAGE: Self = Self(4442);
	pub const EFFECT_GATHER: Self = Self(4446);
}
impl fmt::Display for Ability {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match self.0 {
			0 => "None",
			1 => "Smart",
			2 => "Effect Taunt",
			20 => "Move Turn",
			24 => "Attack Attack Towards",
			25 => "Attack Attack Barrage",
			40 => "Morph Infested Terrans",
			42 => "Effect Explode",
			44 => "Research Interceptor Graviton Catapult",
			45 => "Research Research Interceptor Launch Speed Upgrade",
			46 => "Research Phoenix Anion Pulse Crystals",
			47 => "Research Tempest Range Upgrade",
			48 => "Research Research Void Ray Speed Upgrade",
			49 => "Research Tempest Research Ground Attack Upgrade",
			74 => "Effect Fungal Growth",
			76 => "Effect Guardian Shield",
			110 => "Train Mothership",
			140 => "Effect Feedback",
			146 => "Hallucination Archon",
			148 => "Hallucination Colossus",
			150 => "Hallucination High Templar",
			152 => "Hallucination Immortal",
			154 => "Hallucination Phoenix",
			156 => "Hallucination Probe",
			158 => "Hallucination Stalker",
			160 => "Hallucination Void Ray",
			162 => "Hallucination Warp Prism",
			164 => "Hallucination Zealot",
			171 => "Effect Calldown Mule",
			173 => "Effect Graviton Beam",
			181 => "Effect Spawn Changeling",
			216 => "Research Glial Regeneration",
			217 => "Research Tunneling Claws",
			218 => "Research Roach Supply",
			245 => "Effect Sap Structure",
			249 => "Effect Neural Parasite",
			251 => "Effect Inject Larva",
			255 => "Effect Supply Drop",
			263 => "Research Anabolic Synthesis",
			265 => "Research Chitinous Plating",
			301 => "Attack Attack Warp Prism",
			302 => "Attack Warp Prism Attack Towards",
			303 => "Attack Warp Prism Attack Barrage",
			318 => "Build Command Center",
			319 => "Build Supply Depot",
			320 => "Build Refinery",
			321 => "Build Barracks",
			322 => "Build Engineering Bay",
			323 => "Build Missile Turret",
			324 => "Build Bunker",
			325 => "Terran Build Refinery",
			326 => "Build Sensor Tower",
			327 => "Build Ghost Academy",
			328 => "Build Factory",
			329 => "Build Starport",
			331 => "Build Armory",
			333 => "Build Fusion Core",
			386 => "Effect Heal",
			388 => "Morph Siege Mode",
			390 => "Morph Unsiege",
			399 => "Effect Scan",
			401 => "Effect Yamato Gun",
			403 => "Morph Viking Assault Mode",
			405 => "Morph Viking Fighter Mode",
			524 => "Train Scv",
			556 => "Morph Supply Depot Lower",
			558 => "Morph Supply Depot Raise",
			560 => "Train Marine",
			561 => "Train Reaper",
			562 => "Train Ghost",
			563 => "Train Marauder",
			591 => "Train Siege Tank",
			594 => "Train Thor",
			595 => "Train Hellion",
			596 => "Train Hellbat",
			597 => "Train Cyclone",
			614 => "Train Widow Mine",
			620 => "Train Medivac",
			621 => "Train Banshee",
			622 => "Train Raven",
			623 => "Train Battlecruiser",
			624 => "Train Viking Fighter",
			626 => "Train Liberator",
			650 => "Research Hi Sec Auto Tracking",
			651 => "Research Terran Structure Armor Upgrade",
			655 => "Research Neosteel Frame",
			710 => "Build Nuke",
			730 => "Research Stimpack",
			731 => "Research Combat Shield",
			732 => "Research Concussive Shells",
			761 => "Research Infernal Preigniter",
			763 => "Research Research Transformation Servos",
			764 => "Research Drilling Claws",
			765 => "Research Research Lock On Range Upgrade",
			766 => "Research Smart Servos",
			767 => "Research Research Armor Piercing Rockets",
			768 => "Research Cyclone Rapid Fire Launchers",
			769 => "Research Cyclone Lock On Damage",
			770 => "Research Cyclone Research Hurricane Thrusters",
			790 => "Research Banshee Cloaking Field",
			792 => "Research Research Medivac Energy Upgrade",
			793 => "Research Raven Corvid Reactor",
			796 => "Research Research Seeker Missile",
			797 => "Research Research Durable Materials",
			799 => "Research Banshee Hyperflight Rotors",
			800 => "Research Research Liberator Ag Mode",
			802 => "Research Research Rapid Deployment",
			803 => "Research Raven Recalibrated Explosives",
			804 => "Research High Capacity Fuel Tanks",
			805 => "Research Advanced Ballistics",
			806 => "Research Raven Research Enhanced Munitions",
			807 => "Research Research Raven Interference Matrix",
			820 => "Research Personal Cloaking",
			852 => "Research Terran Vehicle Plating Level 1",
			853 => "Research Terran Vehicle Plating Level 2",
			854 => "Research Terran Vehicle Plating Level 3",
			858 => "Research Terran Ship Plating Level 1",
			859 => "Research Terran Ship Plating Level 2",
			860 => "Research Terran Ship Plating Level 3",
			880 => "Build Nexus",
			881 => "Build Pylon",
			882 => "Build Assimilator",
			883 => "Build Gateway",
			884 => "Build Forge",
			885 => "Build Fleet Beacon",
			886 => "Build Twilight Council",
			887 => "Build Photon Cannon",
			889 => "Build Stargate",
			890 => "Build Templar Archive",
			891 => "Build Dark Shrine",
			892 => "Build Robotics Bay",
			893 => "Build Robotics Facility",
			894 => "Build Cybernetics Core",
			895 => "Build Shield Battery",
			916 => "Train Zealot",
			917 => "Train Stalker",
			919 => "Train High Templar",
			920 => "Train Dark Templar",
			921 => "Train Sentry",
			922 => "Train Adept",
			946 => "Train Phoenix",
			948 => "Train Carrier",
			950 => "Train Void Ray",
			954 => "Train Oracle",
			955 => "Train Tempest",
			976 => "Train Warp Prism",
			977 => "Train Observer",
			978 => "Train Colossus",
			979 => "Train Immortal",
			994 => "Train Disruptor",
			1006 => "Train Probe",
			1036 => "Effect Psi Storm",
			1042 => "Build Interceptors",
			1093 => "Research Gravitic Booster",
			1094 => "Research Gravitic Drive",
			1097 => "Research Extended Thermal Lance",
			1099 => "Research Research Immortal Revive",
			1126 => "Research Psi Storm",
			1152 => "Build Hatchery",
			1153 => "Build Creep Tumor",
			1154 => "Build Extractor",
			1155 => "Build Spawning Pool",
			1156 => "Build Evolution Chamber",
			1157 => "Build Hydralisk Den",
			1158 => "Build Spire",
			1159 => "Build Ultralisk Cavern",
			1160 => "Build Infestation Pit",
			1161 => "Build Nydus Network",
			1162 => "Build Baneling Nest",
			1163 => "Build Lurker Den",
			1165 => "Build Roach Warren",
			1166 => "Build Spine Crawler",
			1167 => "Build Spore Crawler",
			1195 => "Research Evolve Propulsive Peristalsis",
			1216 => "Morph Lair",
			1218 => "Morph Hive",
			1220 => "Morph Greater Spire",
			1223 => "Research Pneumatized Carapace",
			1224 => "Research Evolve Ventral Sacks",
			1225 => "Research Burrow",
			1252 => "Research Zergling Adrenal Glands",
			1253 => "Research Zergling Metabolic Boost",
			1282 => "Research Grooved Spines",
			1283 => "Research Muscular Augments",
			1284 => "Research Research Frenzy",
			1286 => "Research Research Lurker Range",
			1342 => "Train Drone",
			1343 => "Train Zergling",
			1344 => "Train Overlord",
			1345 => "Train Hydralisk",
			1346 => "Train Mutalisk",
			1348 => "Train Ultralisk",
			1351 => "Train Roach",
			1352 => "Train Infestor",
			1353 => "Train Corruptor",
			1354 => "Train Viper",
			1356 => "Train Swarm Host",
			1372 => "Morph Brood Lord",
			1413 => "Train Warp Zealot",
			1414 => "Train Warp Stalker",
			1416 => "Train Warp High Templar",
			1417 => "Train Warp Dark Templar",
			1418 => "Train Warp Sentry",
			1419 => "Train Warp Adept",
			1448 => "Morph Overseer",
			1450 => "Morph Planetary Fortress",
			1455 => "Research Neural Parasite",
			1456 => "Research Research Locust Lifetime Increase",
			1457 => "Research Evolve Amorphous Armorcloud",
			1482 => "Research Centrifugal Hooks",
			1516 => "Morph Orbital Command",
			1518 => "Morph Warp Gate",
			1520 => "Morph Gateway",
			1526 => "Effect Force Field",
			1528 => "Morph Warp Prism Phasing Mode",
			1530 => "Morph Warp Prism Transport Mode",
			1532 => "Research Battlecruiser Weapon Refit",
			1533 => "Research Research Ballistic Range",
			1534 => "Research Research Rapid Reignition System",
			1535 => "Fusion Core Research Research Medivac Energy Upgrade",
			1568 => "Research Warp Gate",
			1571 => "Research Research Hallucination",
			1592 => "Research Charge",
			1593 => "Research Blink",
			1594 => "Research Adept Resonating Glaives",
			1595 => "Research Research Psionic Surge",
			1596 => "Research Research Amplified Shielding",
			1597 => "Research Research Psionic Amplifiers",
			1622 => "Effect Nuke Calldown",
			1628 => "Effect Emp",
			1632 => "Train Queen",
			1664 => "Effect Transfusion",
			1668 => "Morph Tech Lab Barracks",
			1670 => "Morph Tech Lab Factory",
			1672 => "Morph Tech Lab Starport",
			1676 => "Morph Reactor",
			1678 => "Factory Reactor Morph Reactor",
			1680 => "Starport Reactor Morph Reactor",
			1692 => "Behavior Generate Creep On",
			1693 => "Behavior Generate Creep Off",
			1764 => "Effect Auto Turret",
			1766 => "Morph Archon",
			1767 => "Archon Warp Target",
			1768 => "Build Nydus Worm",
			1769 => "Build Summon Nydus Canal Attacker",
			1819 => "Effect Charge",
			1821 => "Effect Herd",
			1825 => "Effect Contaminate",
			1837 => "Morph Move",
			1839 => "Effect Digester Creep Spray",
			1847 => "Morph Mothership",
			1928 => "Effect Xel Naga Healing Shrine",
			1930 => "Effect Nexus Invulnerability",
			1978 => "Morph Hellion",
			1998 => "Morph Hellbat",
			2049 => "Attack Protoss Building Attack Towards",
			2050 => "Attack Protoss Building Attack Barrage",
			2058 => "Stop Hold Fire",
			2059 => "Stop Cheer",
			2060 => "Stop Dance",
			2063 => "Effect Blinding Cloud",
			2067 => "Effect Abduct",
			2073 => "Effect Viper Consume",
			2081 => "Behavior Building Attack On",
			2082 => "Behavior Building Attack Off",
			2083 => "Effect Pickup Scrap Small",
			2085 => "Effect Pickup Scrap Medium",
			2087 => "Effect Pickup Scrap Large",
			2089 => "Effect Pickup Pallet Gas",
			2091 => "Effect Pickup Pallet Minerals",
			2093 => "Effect Massive Knockover",
			2099 => "Effect Widow Mine Attack",
			2101 => "Augment Tornado Missile",
			2114 => "Hallucination Oracle",
			2116 => "Effect Medivac Ignite Afterburners",
			2144 => "Effect Critter Flee",
			2146 => "Effect Oracle Revelation",
			2158 => "Effect Ultralisk Weapon Cooldown",
			2162 => "Effect Photon Overcharge",
			2244 => "Effect Time Warp",
			2324 => "Effect Caustic Spray",
			2330 => "Morph Ravager",
			2332 => "Morph Lurker",
			2338 => "Effect Corrosive Bile",
			2344 => "Effect Purification Nova",
			2346 => "Purification Nova Targeted Purification Nova Targeted",
			2350 => "Effect Lock On",
			2358 => "Effect Tactical Jump",
			2362 => "Morph Thor High Impact Mode",
			2364 => "Morph Thor Explosive Mode",
			2375 => "Behavior Pulsar Beam On",
			2376 => "Behavior Pulsar Beam Off",
			2383 => "Morph Locust Mp Flying Swoop",
			2385 => "Locust Mp Morph To Air Locust Mp Flying Swoop",
			2387 => "Effect Locust Swoop",
			2389 => "Hallucination Disruptor",
			2391 => "Hallucination Adept",
			2393 => "Effect Void Ray Prismatic Alignment",
			2469 => "Morph Immortal",
			2473 => "Effect Arbiter Mp Stasis Field",
			2475 => "Effect Arbiter Mp Recall",
			2477 => "Effect Corsair Mp Disruption Web",
			2479 => "Morph Morph To Guardian Mp",
			2481 => "Morph Morph To Devourer Mp",
			2483 => "Effect Defiler Mp Consume",
			2485 => "Effect Defiler Mp Dark Swarm",
			2487 => "Effect Defiler Mp Plague",
			2493 => "Effect Queen Mp Ensnare",
			2495 => "Effect Queen Mp Spawn Broodlings",
			2497 => "Effect Queen Mp Infest Command Center",
			2505 => "Build Stasis Trap",
			2536 => "Effect Activate Stasis Ward",
			2542 => "Effect Parasitic Bomb",
			2544 => "Effect Adept Phase Shift",
			2548 => "Morph Purification Nova",
			2554 => "Morph Liberator Ag Mode",
			2556 => "Morph Liberator Aa Mode",
			2558 => "Liberator Ag Target Liberator Ag Mode",
			2560 => "Liberator Aa Target Liberator Aa Mode",
			2588 => "Effect Kd 8 Charge",
			2598 => "Effect Slayn Elemental Grab",
			2704 => "Effect Spawn Locusts",
			2706 => "Effect Locust Mp Flying Swoop",
			2708 => "Morph Overlord Transport",
			2714 => "Effect Ghost Snipe",
			2716 => "Morph Mothership Core Weapon",
			2718 => "Purify Morph Pylon Back Mothership Core Weapon",
			2720 => "Research Shadow Strike",
			3659 => "Cancel",
			3660 => "Halt",
			3661 => "Burrow Down",
			3662 => "Burrow Up",
			3663 => "Load All",
			3664 => "Unload All",
			3665 => "Stop",
			3666 => "Harvest Gather",
			3667 => "Harvest Return",
			3668 => "Load",
			3669 => "Unload All At",
			3670 => "Unload Unit",
			3671 => "Cancel Last",
			3672 => "Cancel Slot",
			3673 => "Rally Units",
			3674 => "Attack",
			3675 => "Effect Stim",
			3676 => "Behavior Cloak On",
			3677 => "Behavior Cloak Off",
			3678 => "Land",
			3679 => "Lift",
			3680 => "Morph Root",
			3681 => "Morph Uproot",
			3682 => "Build Tech Lab",
			3683 => "Build Reactor",
			3684 => "Effect Spray",
			3685 => "Effect Repair",
			3686 => "Effect Mass Recall",
			3687 => "Effect Blink",
			3688 => "Behavior Hold Fire On",
			3689 => "Behavior Hold Fire Off",
			3690 => "Rally Workers",
			3691 => "General Build Creep Tumor Build Creep Tumor",
			3692 => "Research Protoss Air Armor",
			3693 => "Research Protoss Air Weapons",
			3694 => "Research Protoss Ground Armor",
			3695 => "Research Protoss Ground Weapons",
			3696 => "Research Protoss Shields",
			3697 => "Research Terran Infantry Armor",
			3698 => "Research Terran Infantry Weapons",
			3699 => "Research Terran Ship Weapons",
			3700 => "Research Terran Vehicle And Ship Plating",
			3701 => "Research Terran Vehicle Weapons",
			3702 => "Research Zerg Flyer Armor",
			3703 => "Research Zerg Flyer Attack",
			3704 => "Research Zerg Ground Armor",
			3705 => "Research Zerg Melee Weapons",
			3706 => "Research Zerg Missile Weapons",
			3709 => "Research Adaptive Talons",
			3710 => "Lurker Den Research Research Lurker Range",
			3739 => "Morph Observer Mode",
			3741 => "Morph Surveillance Mode",
			3743 => "Morph Oversight Mode",
			3745 => "Morph Overseer Mode",
			3747 => "Effect Interference Matrix",
			3753 => "Effect Anti Armor Missile",
			3755 => "Effect Chrono Boost Energy Cost",
			3763 => "Effect Infestor Ensnare",
			3772 => "Battlecruiser Attack Attack Towards",
			3773 => "Battlecruiser Attack Attack Barrage",
			3779 => "Move Acquire Move",
			3780 => "Battlecruiser Move Turn",
			3784 => "Battlecruiser Stop Hold Fire",
			3785 => "Battlecruiser Stop Cheer",
			3786 => "Battlecruiser Stop Dance",
			3789 => "Viper Parasitic Bomb Relay Parasitic Bomb",
			3791 => "Parasitic Bomb Relay Dodge Parasitic Bomb",
			3793 => "Hold Position",
			3794 => "Move",
			3795 => "Patrol",
			3796 => "General Unload Unit Unload Unit",
			4107 => "Effect Battery Overcharge",
			4109 => "Effect Amorphous Armorcloud",
			4111 => "Effect Shield Battery Recharge",
			4119 => "Morph Baneling",
			4122 => "Effect Oracle Cloak Field",
			4440 => "Effect Energy Recharge",
			4442 => "Effect Salvage",
			4446 => "Effect Gather",
			_ => return write!(f, "{self:?}"),
		};
		write!(f, "{s}")
	}
}
impl From<u32> for Ability {
	fn from(n: u32) -> Self {
		Self(n)
	}
}
impl From<Ability> for u32 {
	fn from(id: Ability) -> Self {
		id.0
	}
}
