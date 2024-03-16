use super::*;
use bitflags::bitflags;
use camino::Utf8PathBuf;
use sc2_prost::Race;
use std::net::IpAddr;

/// Game configuration for [`Client::create_game`] request.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GameCfg {
	/// Map for the game. Can be specified as a path or data bytes.
	pub map: common::Load,
	/// Game participants.
	pub participants: Vec<create_game::Participant>,
	/// If set to `true`, fog of war will be disabled for all players
	pub disable_fog: bool,
	/// Can be used to reproduce randomness of the game.
	pub random_seed: u32,
	/// If set to `true`, game will run in realtime mode.
	/// Otherwise game will run in step mode.
	pub realtime: bool,
}
impl From<GameCfg> for Req {
	fn from(cfg: GameCfg) -> Req {
		use common::Load;
		use create_game::Participant::*;
		use sc2_prost::PlayerType;

		Req::CreateGame(sc2_prost::RequestCreateGame {
			map: Some(sc2_prost::request_create_game::Map::LocalMap(
				match cfg.map {
					Load::Path(path) => sc2_prost::LocalMap {
						map_path: path.into(),
						map_data: vec![],
					},
					Load::Data(data) => sc2_prost::LocalMap {
						map_path: "".into(),
						map_data: data,
					},
				},
			)),
			player_setup: cfg
				.participants
				.into_iter()
				.map(|p| match p {
					Player => sc2_prost::PlayerSetup {
						r#type: PlayerType::Participant as i32,
						..Default::default()
					},
					Computer(c) => sc2_prost::PlayerSetup {
						r#type: PlayerType::Computer as i32,
						race: c.race as i32,
						difficulty: c.difficulty as i32,
						player_name: c.name,
						ai_build: c.ai_build as i32,
					},
					Observer => sc2_prost::PlayerSetup {
						r#type: PlayerType::Observer as i32,
						..Default::default()
					},
				})
				.collect(),
			disable_fog: cfg.disable_fog,
			random_seed: cfg.random_seed,
			realtime: cfg.realtime,
		})
	}
}

pub mod create_game {
	use super::*;

	#[derive(Debug, Default, Clone, PartialEq, Eq)]
	pub struct Computer {
		pub race: Race,
		pub difficulty: sc2_prost::Difficulty,
		pub name: String,
		pub ai_build: sc2_prost::AiBuild,
	}

	#[derive(Debug, Clone, PartialEq, Eq)]
	pub enum Participant {
		Player,
		Computer(Computer),
		Observer,
	}
}

/// Player configuration for [`Client::join_game`] request.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct JoinCfg {
	/// Can join as a player with the specified [`Race`]
	/// or as an observer with the specified Id.
	pub join_as: join_game::JoinAs,
	/// Interface config
	pub interface: common::Interface,
	pub server_ports: Option<join_game::Ports>,
	pub client_ports: Vec<join_game::Ports>,
	pub name: String,
	pub host: Option<IpAddr>,
}
impl From<JoinCfg> for Req {
	fn from(cfg: JoinCfg) -> Req {
		use join_game::JoinAs;
		use sc2_prost::request_join_game::Participation::*;

		Req::JoinGame(sc2_prost::RequestJoinGame {
			participation: Some(match cfg.join_as {
				JoinAs::Player(race) => Race(race as i32),
				JoinAs::Observer(id) => ObservedPlayerId(id),
			}),
			options: Some(cfg.interface.into()),
			server_ports: cfg.server_ports.map(Into::into),
			client_ports: cfg.client_ports.into_iter().map(Into::into).collect(),
			shared_port: 0,
			player_name: cfg.name,
			host_ip: cfg.host.map_or_else(String::new, |ip| ip.to_string()),
		})
	}
}

pub mod join_game {
	use super::*;

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub enum JoinAs {
		Player(Race),
		Observer(u32),
	}
	impl Default for JoinAs {
		fn default() -> Self {
			Self::Player(Default::default())
		}
	}
	impl From<Race> for JoinAs {
		fn from(race: Race) -> Self {
			Self::Player(race)
		}
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct Ports {
		pub game: u16,
		pub base: u16,
	}
	impl Ports {
		pub fn new(game: u16, base: u16) -> Self {
			Self { game, base }
		}
	}
	impl From<(u16, u16)> for Ports {
		fn from((game, base): (u16, u16)) -> Self {
			Self { game, base }
		}
	}
	impl From<Ports> for sc2_prost::PortSet {
		fn from(p: Ports) -> Self {
			Self {
				game_port: p.game as i32,
				base_port: p.base as i32,
			}
		}
	}
}

pub mod common {
	use super::*;

	#[derive(Debug, Default, Clone, PartialEq)]
	pub struct Interface {
		pub flags: Flags,
		pub feature: Option<sc2_prost::SpatialCameraSetup>,
		pub render: Option<sc2_prost::SpatialCameraSetup>,
	}
	impl From<Interface> for sc2_prost::InterfaceOptions {
		fn from(i: Interface) -> Self {
			Self {
				raw: i.flags.contains(Flags::RAW),
				score: i.flags.contains(Flags::SCORE),
				feature_layer: i.feature,
				render: i.render,
				show_cloaked: i.flags.contains(Flags::CLOAKED),
				show_burrowed_shadows: i.flags.contains(Flags::BURROWED),
				show_placeholders: i.flags.contains(Flags::PLACEHOLDERS),
				raw_affects_selection: i.flags.contains(Flags::AFFECT_SELECTION),
				raw_crop_to_playable_area: i.flags.contains(Flags::CROP_RAW),
			}
		}
	}

	bitflags! {
		#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
		#[repr(transparent)]
		pub struct Flags: u8 {
			const RAW              = 1;
			const SCORE            = 1 << 1;
			const CLOAKED          = 1 << 2;
			const BURROWED         = 1 << 3;
			const PLACEHOLDERS     = 1 << 4;
			const AFFECT_SELECTION = 1 << 5;
			const CROP_RAW         = 1 << 6;
		}
	}
	impl Default for Flags {
		fn default() -> Self {
			Self::RAW | Self::CLOAKED | Self::BURROWED | Self::AFFECT_SELECTION | Self::CROP_RAW
		}
	}

	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	pub enum Load {
		Path(Utf8PathBuf),
		Data(Vec<u8>),
	}
	impl Default for Load {
		fn default() -> Self {
			Load::Path(<_>::default())
		}
	}
	impl<T: Into<Utf8PathBuf>> From<T> for Load {
		fn from(path: T) -> Self {
			Load::Path(path.into())
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ReplayCfg {
	pub replay: common::Load,
	pub map_data: Vec<u8>,
	pub player: u32,
	pub interface: common::Interface,
	pub disable_fog: bool,
	pub realtime: bool,
	pub record: bool,
}
impl From<ReplayCfg> for Req {
	fn from(cfg: ReplayCfg) -> Req {
		use common::Load;
		use sc2_prost::request_start_replay::Replay::*;
		Req::StartReplay(sc2_prost::RequestStartReplay {
			replay: Some(match cfg.replay {
				Load::Path(path) => ReplayPath(path.into()),
				Load::Data(data) => ReplayData(data),
			}),
			map_data: cfg.map_data,
			observed_player_id: cfg.player as i32,
			options: Some(cfg.interface.into()),
			disable_fog: cfg.disable_fog,
			realtime: cfg.realtime,
			record_replay: cfg.record,
		})
	}
}

bitflags! {
	#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
	#[repr(transparent)]
	pub struct DataFlags: u8 {
		const ABILITIES = 1;
		const UNITS     = 1 << 1;
		const UPGRADES  = 1 << 2;
		const BUFFS     = 1 << 3;
		const EFFECTS   = 1 << 4;
	}
}
impl From<DataFlags> for Req {
	fn from(flags: DataFlags) -> Req {
		Req::Data(sc2_prost::RequestData {
			ability_id: flags.contains(DataFlags::ABILITIES),
			unit_type_id: flags.contains(DataFlags::UNITS),
			upgrade_id: flags.contains(DataFlags::UPGRADES),
			buff_id: flags.contains(DataFlags::BUFFS),
			effect_id: flags.contains(DataFlags::EFFECTS),
		})
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ReplayInfoCfg {
	pub replay: common::Load,
	pub download_data: bool,
}
impl From<ReplayInfoCfg> for Req {
	fn from(cfg: ReplayInfoCfg) -> Req {
		use common::Load;
		use sc2_prost::request_replay_info::Replay::*;
		Req::ReplayInfo(sc2_prost::RequestReplayInfo {
			replay: Some(match cfg.replay {
				Load::Path(path) => ReplayPath(path.into()),
				Load::Data(data) => ReplayData(data),
			}),
			download_data: cfg.download_data,
		})
	}
}

macro_rules! unwrap_data {
	($res:expr; $Var:ident $($field:ident)?) => {
		$res.map(|res| {
			res.map(|data| {
				let ResVar::$Var(data) = data else {
					unreachable!()
				};
				data $(.$field)?
			})
		})
	};
}
macro_rules! simple_methods {
	($( $(#[$attr:meta])* $name:ident $Var:ident ),+ $(,)?) => {$(
		$(#[$attr])*
		pub fn $name(&mut self) -> Result<Res<()>> {
			self.send(Req::$Var(<_>::default())).map(empty_res)
		}
	)+};
}

impl Client {
	/**
	Sends [`CreateGame`](Req::CreateGame) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestCreateGame { /* Game config */ };
	let res = client.send(Req::CreateGame(req))?;
	```

	# Examples
	Single player game vs computer:
	```no_run
	use sc2_core::request::{
		create_game::{Computer, Participant},
		GameCfg,
	};
	use sc2_prost::{AiBuild, Difficulty};

	let cfg = GameCfg {
		map: "Test.SC2Map".into(),
		participants: vec![
			Participant::Player,
			Participant::Computer(Computer {
				difficulty: Difficulty::VeryHard,
				ai_build: AiBuild::Rush,
				..Default::default()
			}),
		],
		..Default::default()
	};
	let res = client.create_game(cfg)?;
	```
	*/
	pub fn create_game(&mut self, cfg: GameCfg) -> Result<Res<()>> {
		self.send(cfg.into()).map(empty_res)
	}
	/**
	Sends [`JoinGame`](Req::JoinGame) request to the server.
	Returns [`player_id`] in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestJoinGame { /* Join config */ };
	let res = client.send(Req::JoinGame(req))?;
	let ResVar::JoinGame(data) = res.data else { unreachable!() };
	let player_id = data.player_id;
	```

	# Examples

	Join single player game:
	```no_run
	use sc2_core::request::JoinCfg;
	use sc2_prost::Race;

	let cfg = JoinCfg {
		join_as: Race::Terran.into(),
		name: "TestBot".into(),
		..Default::default()
	};
	let res = client.join_game(cfg)?;
	println!("Our player_id: {}", res.data);
	```

	Join multi player game:
	```no_run
	use sc2_core::request::JoinCfg;
	use sc2_prost::Race;

	let cfg = JoinCfg {
		join_as: Race::Terran.into(),
		server_ports: Some((5001, 5002).into()),
		client_ports: vec![(5003, 5004).into(), (5005, 5006).into()],
		host: "127.0.0.1:5000".parse().ok(),
		..Default::default()
	};
	let res = client.join_game(cfg)?;
	println!("Our player_id: {}", res.data);
	```

	[`player_id`]: sc2_prost::ResponseJoinGame::player_id
	*/
	pub fn join_game(&mut self, cfg: JoinCfg) -> Result<Res<u32>> {
		unwrap_data!(self.send(cfg.into()); JoinGame player_id)
	}
	/**
	Sends [`RestartGame`](Req::RestartGame) request to the server.
	Returns [`need_hard_reset`] flag in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::RestartGame(Default::default()))?;
	let ResVar::RestartGame(data) = res.data else { unreachable!() };
	let need_hard_reset = data.need_hard_reset;
	```

	[`need_hard_reset`]: sc2_prost::ResponseRestartGame::need_hard_reset
	*/
	pub fn restart_game(&mut self) -> Result<Res<bool>> {
		unwrap_data!(self.send(Req::RestartGame(<_>::default())); RestartGame need_hard_reset)
	}
	/**
	Sends [`StartReplay`](Req::StartReplay) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestStartReplay { /* Replay config */ };
	let res = client.send(Req::StartReplay(req))?;
	```
	*/
	pub fn start_replay(&mut self, cfg: ReplayCfg) -> Result<Res<()>> {
		self.send(cfg.into()).map(empty_res)
	}
	simple_methods! {
		/**
		Sends [`LeaveGame`](Req::LeaveGame) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.send(Req::LeaveGame(Default::default()))?;
		```
		*/
		leave_game LeaveGame,
		/**
		Sends [`QuickSave`](Req::QuickSave) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.send(Req::QuickSave(Default::default()))?;
		```
		*/
		quick_save QuickSave,
		/**
		Sends [`QuickLoad`](Req::QuickLoad) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.send(Req::QuickLoad(Default::default()))?;
		```
		*/
		quick_load QuickLoad,
		/**
		Sends [`Quit`](Req::Quit) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.send(Req::Quit(Default::default()))?;
		```
		*/
		quit Quit,
	}
	/**
	Sends [`Observation`](Req::Observation) request to the server.
	Returns observation in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestObservation { /* Observation options */ };
	let res = client.send(Req::Observation(req))?;
	let ResVar::Observation(data) = res.data else { unreachable!() };
	```
	*/
	pub fn observation(
		&mut self,
		cfg: sc2_prost::RequestObservation,
	) -> Result<Res<sc2_prost::ResponseObservation>> {
		unwrap_data!(self.send(Req::Observation(cfg)); Observation)
	}
	/**
	Sends [`Action`](Req::Action) request to the server.
	Returns action results in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestAction { actions: vec![] };
	let res = client.send(Req::Action(req))?;
	let ResVar::Action(data) = res.data else { unreachable!() };
	let action_results = data.result;
	```
	*/
	pub fn action(
		&mut self,
		acts: Vec<sc2_prost::Action>,
	) -> Result<Res<Vec<sc2_prost::ActionResult>>> {
		let res = self.send(Req::Action(sc2_prost::RequestAction { actions: acts }));
		Ok(unwrap_data!(res; Action result)?.map(|result| {
			result
				.into_iter()
				.map(|num| num.try_into().unwrap_or_default())
				.collect()
		}))
	}
	/**
	Sends [`ObsAction`](Req::ObsAction) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestObserverAction { actions: vec![] };
	let res = client.send(Req::ObsAction(req))?;
	```
	*/
	pub fn obs_action(&mut self, acts: Vec<sc2_prost::observer_action::Action>) -> Result<Res<()>> {
		self.send(Req::ObsAction(sc2_prost::RequestObserverAction {
			actions: acts
				.into_iter()
				.map(|act| sc2_prost::ObserverAction { action: Some(act) })
				.collect(),
		}))
		.map(empty_res)
	}
	/**
	Sends [`Step`](Req::Step) request to the server.
	Returns [`simulation_loop`] in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::Step(sc2_prost::RequestStep { count: 2 }))?;
	let ResVar::Step(data) = res.data else { unreachable!() };
	let simulation_loop = data.simulation_loop;
	```

	[`simulation_loop`]: sc2_prost::ResponseStep::simulation_loop
	*/
	pub fn step(&mut self, count: u32) -> Result<Res<u32>> {
		let res = self.send(Req::Step(sc2_prost::RequestStep { count }));
		unwrap_data!(res; Step simulation_loop)
	}
	/**
	Sends [`Data`](Req::Data) request to the server.
	Returns Ids data in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestData { /* Data options */ };
	let res = client.send(Req::Data(req))?;
	let ResVar::Data(data) = res.data else { unreachable!() };
	```
	# Examples
	```no_run
	use sc2_core::request::DataFlags;

	let res = client.data(DataFlags::all())?;
	```
	*/
	pub fn data(&mut self, flags: DataFlags) -> Result<Res<sc2_prost::ResponseData>> {
		unwrap_data!(self.send(flags.into()); Data)
	}
	/**
	Sends [`Query`](Req::Query) request to the server.
	Returns query results in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestQuery { /* Query fields */ };
	let res = client.send(Req::Query(req))?;
	let ResVar::Query(data) = res.data else { unreachable!() };
	```
	*/
	pub fn query(&mut self, cfg: sc2_prost::RequestQuery) -> Result<Res<sc2_prost::ResponseQuery>> {
		unwrap_data!(self.send(Req::Query(cfg)); Query)
	}
	/**
	Sends [`SaveReplay`](Req::SaveReplay) request to the server.
	Returns replay data in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::SaveReplay(Default::default()))?;
	let ResVar::SaveReplay(data) = res.data else { unreachable!() };
	```
	*/
	pub fn save_replay(&mut self) -> Result<Res<Vec<u8>>> {
		unwrap_data!(self.send(Req::SaveReplay(<_>::default())); SaveReplay data)
	}
	/**
	Sends [`MapCommand`](Req::MapCommand) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestMapCommand { trigger_cmd: "Some map command".into() };
	let res = client.send(Req::MapCommand(req))?;
	```
	*/
	pub fn map_command(&mut self, cmd: String) -> Result<Res<()>> {
		self.send(Req::MapCommand(sc2_prost::RequestMapCommand {
			trigger_cmd: cmd,
		}))
		.map(empty_res)
	}
	/**
	Sends [`ReplayInfo`](Req::ReplayInfo) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestReplayInfo { /* Options */ };
	let res = client.send(Req::ReplayInfo(req))?;
	let ResVar::ReplayInfo(data) = res.data else { unreachable!() };
	```
	*/
	pub fn replay_info(
		&mut self,
		cfg: ReplayInfoCfg,
	) -> Result<Res<sc2_prost::ResponseReplayInfo>> {
		unwrap_data!(self.send(cfg.into()); ReplayInfo)
	}
	/**
	Sends [`AvailableMaps`](Req::AvailableMaps) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::AvailableMaps(Default::default()))?;
	let ResVar::AvailableMaps(data) = res.data else { unreachable!() };
	```
	*/
	pub fn available_maps(&mut self) -> Result<Res<sc2_prost::ResponseAvailableMaps>> {
		unwrap_data!(self.send(Req::AvailableMaps(<_>::default())); AvailableMaps)
	}
	/**
	Sends [`SaveMap`](Req::SaveMap) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestSaveMap { /* Save config */ };
	let res = client.send(Req::SaveMap(req))?;
	```
	*/
	pub fn save_map(&mut self, cfg: sc2_prost::RequestSaveMap) -> Result<Res<()>> {
		self.send(Req::SaveMap(cfg)).map(empty_res)
	}
	/**
	Sends [`Ping`](Req::Ping) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::Ping(Default::default()))?;
	let ResVar::Ping(data) = res.data else { unreachable!() };
	```
	*/
	pub fn ping(&mut self) -> Result<Res<sc2_prost::ResponsePing>> {
		unwrap_data!(self.send(Req::Ping(<_>::default())); Ping)
	}
	/**
	Sends [`Debug`](Req::Debug) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestDebug { debug: vec![/* Debug commands */] };
	let res = client.send(Req::Debug(req))?;
	```
	*/
	pub fn debug(&mut self, cmds: Vec<sc2_prost::debug_command::Command>) -> Result<Res<()>> {
		self.send(Req::Debug(sc2_prost::RequestDebug {
			debug: cmds
				.into_iter()
				.map(|cmd| sc2_prost::DebugCommand { command: Some(cmd) })
				.collect(),
		}))
		.map(empty_res)
	}
}

fn empty_res(res: Res) -> Res<()> {
	res.map(|_| ())
}
