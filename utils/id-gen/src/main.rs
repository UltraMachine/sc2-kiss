// #![allow(dead_code, unused_imports, unused_variables)]
use bpaf::{construct, long, Bpaf, Parser};
use camino::Utf8PathBuf;
use convert_case::{Case, Casing};
use sc2_core::{
	client::{Client, Result},
	instance::Launcher,
	request::{create_game::Participant, DataFlags, GameCfg},
};
use std::{
	collections::HashSet,
	fs::File,
	io::{self, BufReader, BufWriter, Write},
	net::{IpAddr, Ipv6Addr, SocketAddr},
	path::PathBuf,
	thread::sleep,
	time::Duration,
};

const LOCALHOST_5000: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 5000);

fn default_out_dir() -> Result<PathBuf, std::convert::Infallible> {
	Ok("kiss/src/ids".into())
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Cli {
	#[bpaf(external)]
	input: Input,
	#[bpaf(argument("PATH"))]
	dump: Option<PathBuf>,
	#[bpaf(argument("PATH"), fallback_with(default_out_dir))]
	out_dir: PathBuf,
	no_gen: bool,
}
#[derive(Debug, Clone, Bpaf)]
enum Input {
	Map {
		#[bpaf(argument("PATH"))]
		map: Utf8PathBuf,
		#[bpaf(argument("IP:PORT"), fallback(LOCALHOST_5000))]
		addr: SocketAddr,
		#[bpaf(external)]
		launch: Option<PathBuf>,
	},
	Data {
		#[bpaf(argument("PATH"))]
		data: PathBuf,
	},
}

fn launch() -> impl Parser<Option<PathBuf>> {
	let flag = long("launch")
		.req_flag(())
		.map(|_| "/games/StarCraft II".into())
		.hide();
	let arg = long("launch")
		.argument("GAME_DIR")
		.help("Launches SC2 Instance");
	construct!([arg, flag]).optional()
}

fn main() {
	let opts = cli().max_width(usize::MAX).fallback_to_usage().run();
	gen(opts).unwrap_or_else(|e| eprintln!("{e}"))
}

fn gen(opts: Cli) -> Result {
	// load data
	let data = match opts.input {
		Input::Map { map, addr, launch } => {
			let _instance = launch.map(|game_dir| {
				Launcher {
					game_dir,
					addr,
					..<_>::default()
				}
				.kill_on_drop()
				.spawn()
				.expect("Can't launch SC2")
			});

			// give some time for SC2 to start
			sleep(Duration::from_secs(3));

			let mut client = Client::connect_addr(addr)?;

			client.create_game(GameCfg {
				map: map.into(),
				participants: vec![Participant::Player],
				..<_>::default()
			})?;
			client.join_game(<_>::default())?;

			client.data(DataFlags::all())?.data
		}
		Input::Data { data } => {
			let file = BufReader::new(File::open(&data).expect("Can't open data"));
			if data.extension() == Some("json".as_ref()) {
				serde_json::from_reader(file).expect("Can't deserialize data")
			} else {
				ron::de::from_reader(file).expect("Can't deserialize data")
			}
		}
	};

	// dump data
	if let Some(dump) = opts.dump {
		let file = BufWriter::new(File::create(&dump).expect("Can't create dump file"));
		if dump.extension() == Some("json".as_ref()) {
			serde_json::to_writer_pretty(file, &data).expect("Can't serialize data");
		} else {
			ron::ser::to_writer_pretty(
				file,
				&data,
				ron::ser::PrettyConfig::new()
					.depth_limit(3)
					.new_line("\n".into())
					.indentor("\t".into()),
			)
			.expect("Can't serialize data");
		}
	}

	if opts.no_gen {
		return Ok(());
	}
	let out_dir = opts.out_dir;
	// fs::create_dir_all(&out_dir).expect("Can't create output directory");

	// codegen
	let mut abil_names = HashSet::new();
	let abils = data
		.abilities
		.into_iter()
		.filter(|abil| {
			abil.available && abil.remaps_to_ability_id == 0 && !abil.button_name.is_empty()
		})
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
		.filter(|unit| {
			unit.available
				&& !(unit.name.ends_with("Dummy")
					|| unit.name.contains("Weapon")
					|| unit.name.ends_with("Missile")
					|| unit.name.starts_with("Shape")
					|| unit.name.starts_with("LoadOutSpray"))
		})
		.map(|unit| (unit.name, unit.unit_id));
	make_ids(out_dir.clone(), "unit_kind", units).expect("Failed to make unit ids");

	let upgrades = data
		.upgrades
		.into_iter()
		.filter(|up| up.ability_id != 0)
		.map(|up| (up.name, up.upgrade_id));
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

	let type_name = file_name.to_case(Case::Pascal);
	writeln!(
		file,
		"\
use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct {}(pub u32);
impl {0} {{
	pub const NONE: Self = Self(0);",
		type_name
	)?;
	let mut ids_copy = vec![];
	for (name, id) in ids {
		if name.chars().next().map_or(true, |c| c.is_ascii_digit())
			|| name.starts_with("Dummy")
			|| name.contains("Bridge")
			|| name.contains("Door")
		{
			continue;
		}
		let name = name.replace('@', "").to_case(Case::UpperSnake);
		writeln!(file, "\tpub const {name}: Self = Self({id});")?;
		ids_copy.push((name, id));
	}
	writeln!(file, "}}")?;

	// Display impl
	writeln!(
		file,
		"\
impl fmt::Display for {type_name} {{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
		let s = match self.0 {{
			0 => \"None\","
	)?;
	for (name, id) in ids_copy {
		let s = name.to_case(Case::Title);
		writeln!(file, "\t\t\t{id} => \"{s}\",")?;
	}
	writeln!(
		file,
		"\t\t\t\
			_ => return write!(f, \"{{self:?}}\"),
		}};
		write!(f, \"{{s}}\")
	}}
}}"
	)?;

	// From/Into u32 conversion
	writeln!(
		file,
		"\
impl From<u32> for {} {{
	fn from(n: u32) -> Self {{
		Self(n)
	}}
}}
impl From<{0}> for u32 {{
	fn from(id: {0}) -> Self {{
		id.0
	}}
}}",
		type_name
	)?;

	Ok(())
}
