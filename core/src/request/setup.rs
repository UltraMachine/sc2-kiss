use super::*;

macro_rules! simple_methods {
	($( $(#[$attr:meta])* $name:ident $Var:ident ),+ $(,)?) => {$(
		$(#[$attr])*
		pub fn $name(&mut self) -> Result<Res<()>> {
			self.send(Req::$Var(<_>::default())).map(empty_res)
		}
	)+};
}

/// Game setup
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
		host: "127.0.0.1".parse().ok(),
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
}

/// Game configuration for [`Client::create_game`] request.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GameCfg {
	/// Map for the game. Can be specified as a path or data bytes.
	pub map: LoadMap,
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
		use create_game::Participant::*;
		use sc2_prost::PlayerType;

		Req::CreateGame(sc2_prost::RequestCreateGame {
			map: Some(sc2_prost::request_create_game::Map::LocalMap(
				match cfg.map {
					LoadMap::Path(path) => sc2_prost::LocalMap {
						map_path: path.into(),
						map_data: vec![],
					},
					LoadMap::Data(data) => sc2_prost::LocalMap {
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
	pub interface: Interface,
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ReplayCfg {
	pub replay: LoadMap,
	pub map_data: Vec<u8>,
	pub player: u32,
	pub interface: Interface,
	pub disable_fog: bool,
	pub realtime: bool,
	pub record: bool,
}
impl From<ReplayCfg> for Req {
	fn from(cfg: ReplayCfg) -> Req {
		use sc2_prost::request_start_replay::Replay::*;
		Req::StartReplay(sc2_prost::RequestStartReplay {
			replay: Some(match cfg.replay {
				LoadMap::Path(path) => ReplayPath(path.into()),
				LoadMap::Data(data) => ReplayData(data),
			}),
			map_data: cfg.map_data,
			observed_player_id: cfg.player,
			options: Some(cfg.interface.into()),
			disable_fog: cfg.disable_fog,
			realtime: cfg.realtime,
			record_replay: cfg.record,
		})
	}
}
