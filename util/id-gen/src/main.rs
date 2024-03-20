use convert_case::{Case, Casing};
use sc2_core::request::create_game::Participant;
use sc2_core::request::*;
use sc2_core::*;
use std::{
	collections::HashSet,
	env, fs,
	fs::File,
	io,
	io::{BufReader, BufWriter, Write},
	path::PathBuf,
};

fn main() {
	main2().unwrap_or_else(|e| eprintln!("{e}"))
}

fn main2() -> Result {
	let mut args = env::args();
	let (load_map, path) = match args.nth(1).as_deref() {
		Some("--map") => (true, args.next().expect("Must specify map")),
		Some("--data") => (false, args.next().expect("Must specify path to data")),
		_ => panic!("Must specify --map or --data"),
	};
	let out_dir: PathBuf = args.next().expect("Must specify output directory").into();
	let dump = args
		.next()
		.filter(|arg| arg == "--dump")
		.map(|_| args.next().expect("Must specify dump output file"));

	let data = if load_map {
		let mut client = Client::connect("ws://localhost:5000/sc2api")?;

		client.create_game(GameCfg {
			map: path.into(),
			participants: vec![Participant::Player],
			..<_>::default()
		})?;
		client.join_game(<_>::default())?;

		client.data(DataFlags::all())?.data
	} else {
		let file = BufReader::new(File::open(path).expect("Can't open data"));
		ron::de::from_reader(file).expect("Can't deserialize data")
	};

	if let Some(dump) = dump {
		let file = BufWriter::new(File::create(dump).expect("Can't create dump file"));
		ron::ser::to_writer_pretty(
			file,
			&data,
			ron::ser::PrettyConfig::new()
				.depth_limit(5)
				.indentor("\t".into()),
		)
		.expect("Can't serialize data");
	}

	fs::create_dir_all(&out_dir).expect("Can't create output directory");

	let mut abil_names = HashSet::new();
	let abils = data
		.abilities
		.into_iter()
		.filter(|abil| abil.available && abil.remaps_to_ability_id == 0)
		.map(|abil| {
			(
				if abil_names.insert(abil.friendly_name.clone()) {
					abil.friendly_name
				} else {
					abil.link_name + &abil.button_name
				},
				abil.ability_id,
			)
		});
	make_ids(out_dir.clone(), "ability", abils).expect("Failed to make ability ids");

	let units = data
		.units
		.into_iter()
		.filter(|unit| unit.available)
		.map(|unit| (unit.name, unit.unit_id));
	make_ids(out_dir.clone(), "unit", units).expect("Failed to make unit ids");

	let upgrades = data.upgrades.into_iter().map(|up| (up.name, up.upgrade_id));
	make_ids(out_dir.clone(), "upgrade", upgrades).expect("Failed to make upgrade ids");

	let buffs = data.buffs.into_iter().map(|buff| (buff.name, buff.buff_id));
	make_ids(out_dir.clone(), "buff", buffs).expect("Failed to make buff ids");

	let effects = data
		.effects
		.into_iter()
		.map(|ef| (ef.friendly_name, ef.effect_id));
	make_ids(out_dir, "effect", effects).expect("Failed to make effect ids");

	Ok(())
}

fn make_ids(
	mut out_dir: PathBuf,
	file_name: &str,
	ids: impl IntoIterator<Item = (String, u32)>,
) -> io::Result<()> {
	out_dir.push(file_name);
	out_dir.set_extension("rs");
	let mut file = BufWriter::new(File::create(out_dir)?);

	writeln!(file, "use num_enum::{{FromPrimitive, IntoPrimitive}};")?;
	writeln!(file)?;
	let derives =
		"#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, IntoPrimitive)]";
	writeln!(file, "{derives}")?;
	writeln!(file, "#[repr(u32)]")?;
	writeln!(file, "pub enum {} {{", file_name.to_case(Case::Pascal))?;
	writeln!(file, "\t#[default]")?;
	writeln!(file, "\tNone = 0,")?;
	for (name, id) in ids {
		if let Some(c) = name.chars().next() {
			if name.starts_with("Dummy") {
				continue;
			}
			write!(file, "\t")?;
			if c.is_ascii_digit() {
				write!(file, "_")?;
			}
			writeln!(file, "{} = {id},", name.to_case(Case::Pascal))?;
		}
	}
	writeln!(file, "}}")?;
	Ok(())
}
