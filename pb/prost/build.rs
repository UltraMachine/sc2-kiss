use std::path::Path;

fn main() {
	let protos_dir = Path::new("../s2client-proto/s2clientprotocol");
	println!("cargo:rerun-if-changed={}", protos_dir.display());

	let mut proto_file = protos_dir.to_owned();
	proto_file.push("sc2api.proto");

	let serde = "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]";
	let serde_with = |path| format!("#[cfg_attr(feature = \"serde\", serde(with = \"{path}\"))]");
	let serde_skip = |condition, default| {
		format!("#[cfg_attr(feature = \"serde\", serde(skip_serializing_if = \"{condition}\", default = \"{default}\"))]")
	};
	let skip_default = serde_skip("srd::is_default", "Default::default");
	let serde_skip_true = serde_skip("bool::clone", "srd::t");

	let mut config = prost_build::Config::new();
	config
		.boxed("Observation.feature_layer_data")
		.boxed("Observation.score")
		.boxed("Observation.ui_data")
		.message_attribute("RequestCreateGame", "#[derive(Eq, Hash)]")
		.message_attribute("ResponseData", serde)
		.message_attribute("AbilityData", serde)
		.field_attribute("AbilityData.available", &serde_skip_true)
		.enum_attribute("AbilityData.Target", serde)
		.field_attribute("AbilityData.target", serde_with("srd::target"))
		.field_attribute(
			"AbilityData.target",
			serde_skip("srd::is_none_target", "srd::i1"),
		)
		.message_attribute("UnitTypeData", serde)
		.field_attribute("UnitTypeData.available", &serde_skip_true)
		.enum_attribute("Race", serde)
		.field_attribute("UnitTypeData.race", serde_with("srd::race"))
		.enum_attribute("Attribute", serde)
		.field_attribute("UnitTypeData.attributes", serde_with("srd::attributes"))
		.message_attribute("Weapon", serde)
		.enum_attribute("Weapon.TargetType", serde)
		.field_attribute("Weapon.target", serde_with("srd::weapon_target"))
		.field_attribute("Weapon.attacks", serde_skip("srd::is1", "srd::u1"))
		.message_attribute("DamageBonus", serde)
		.field_attribute("DamageBonus.attribute", serde_with("srd::attribute"))
		.message_attribute("UpgradeData", serde)
		.message_attribute("BuffData", serde)
		.message_attribute("EffectData", serde);

	let mut serde_skip_default_fields = |item, fields: &[&str]| {
		for field in fields {
			config.field_attribute(format!("{item}.{field}"), &skip_default);
		}
	};
	serde_skip_default_fields(
		"AbilityData",
		&[
			"link_name",
			"link_index",
			"button_name",
			"friendly_name",
			"hotkey",
			"remaps_to_ability_id",
			"allow_minimap",
			"allow_autocast",
			"is_building",
			"footprint_radius",
			"is_instant_placement",
			"cast_range",
		],
	);
	serde_skip_default_fields(
		"UnitTypeData",
		&[
			"name",
			"cargo_size",
			"mineral_cost",
			"vespene_cost",
			"food_required",
			"food_provided",
			"ability_id",
			"race",
			"build_time",
			"has_vespene",
			"has_minerals",
			"sight_range",
			"tech_alias",
			"unit_alias",
			"tech_requirement",
			"require_attached",
			"attributes",
			"movement_speed",
			"armor",
			"weapons",
		],
	);
	serde_skip_default_fields("Weapon", &["damage_bonus"]);
	serde_skip_default_fields(
		"UpgradeData",
		&[
			"name",
			"mineral_cost",
			"vespene_cost",
			"research_time",
			"ability_id",
		],
	);
	serde_skip_default_fields("BuffData", &["name"]);
	serde_skip_default_fields("EffectData", &["name", "friendly_name", "radius"]);

	config
		.compile_protos(&[proto_file], &[protos_dir.parent().unwrap()])
		.unwrap_or_else(|e| panic!("{e}"))
}
