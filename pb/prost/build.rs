use std::path::Path;

fn main() {
	let protos_dir = Path::new("../s2client-proto/s2clientprotocol");
	println!("cargo:rerun-if-changed={}", protos_dir.display());

	let mut proto_file = protos_dir.to_owned();
	proto_file.push("sc2api.proto");

	let serde = "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]";
	let serde_with = |path| format!("#[cfg_attr(feature = \"serde\", serde(with = \"{path}\"))]");
	prost_build::Config::new()
		.boxed("Observation.feature_layer_data")
		.boxed("Observation.score")
		.boxed("Observation.ui_data")
		.message_attribute("RequestCreateGame", "#[derive(Eq, Hash)]")
		.message_attribute("ResponseData", serde)
		.message_attribute("AbilityData", serde)
		.enum_attribute("AbilityData.Target", serde)
		.field_attribute("AbilityData.target", serde_with("srd::target"))
		.message_attribute("UnitTypeData", serde)
		.enum_attribute("Race", serde)
		.field_attribute("UnitTypeData.race", serde_with("srd::race"))
		.enum_attribute("Attribute", serde)
		.field_attribute("UnitTypeData.attributes", serde_with("srd::attributes"))
		.message_attribute("Weapon", serde)
		.enum_attribute("Weapon.TargetType", serde)
		.field_attribute("Weapon.target", serde_with("srd::weapon_target"))
		.message_attribute("DamageBonus", serde)
		.field_attribute("DamageBonus.attribute", serde_with("srd::attribute"))
		.message_attribute("UpgradeData", serde)
		.message_attribute("BuffData", serde)
		.message_attribute("EffectData", serde)
		.compile_protos(&[proto_file], &[protos_dir.parent().unwrap()])
		.unwrap_or_else(|e| panic!("{e}"))
}
