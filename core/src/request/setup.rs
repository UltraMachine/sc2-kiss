use super::*;
use sc2_prost::{Race, RequestCreateGame, RequestJoinGame, RequestStartReplay};
use std::net::IpAddr;

macro_rules! simple_requests {
	($( $(#[$attr:meta])* $name:ident $Var:ident ),+ $(,)?) => {$(
		$(#[$attr])*
		pub fn $name(&mut self) -> Result<Res<()>> {
			self.request(Req::$Var(Default::default())).map(empty_res)
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
	let res = client.request(Req::CreateGame(req))?;
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
	pub fn create_game(&mut self, cfg: impl Into<RequestCreateGame>) -> Result<Res<()>> {
		request!(self.CreateGame(cfg.into())).map(empty_res)
	}
	/**
	Sends [`JoinGame`](Req::JoinGame) request to the server.
	Returns [`player_id`] in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestJoinGame { /* Join config */ };
	let res = client.request(Req::JoinGame(req))?;
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
	pub fn join_game(&mut self, cfg: impl Into<RequestJoinGame>) -> Result<Res<PlayerId>> {
		request!(self.JoinGame(cfg.into()).player_id).map_res(Into::into)
	}
	/**
	Sends [`RestartGame`](Req::RestartGame) request to the server.
	Returns [`need_hard_reset`] flag in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.request(Req::RestartGame(Default::default()))?;
	let ResVar::RestartGame(data) = res.data else { unreachable!() };
	let need_hard_reset = data.need_hard_reset;
	```

	[`need_hard_reset`]: sc2_prost::ResponseRestartGame::need_hard_reset
	*/
	pub fn restart_game(&mut self) -> Result<Res<bool>> {
		request!(self.RestartGame.need_hard_reset)
	}
	/**
	Sends [`StartReplay`](Req::StartReplay) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestStartReplay { /* Replay config */ };
	let res = client.request(Req::StartReplay(req))?;
	```
	*/
	pub fn start_replay(&mut self, cfg: impl Into<RequestStartReplay>) -> Result<Res<()>> {
		request!(self.StartReplay(cfg.into())).map(empty_res)
	}
	simple_requests! {
		/**
		Sends [`LeaveGame`](Req::LeaveGame) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.request(Req::LeaveGame(Default::default()))?;
		```
		*/
		leave_game LeaveGame,
		/**
		Sends [`QuickSave`](Req::QuickSave) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.request(Req::QuickSave(Default::default()))?;
		```
		*/
		quick_save QuickSave,
		/**
		Sends [`QuickLoad`](Req::QuickLoad) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.request(Req::QuickLoad(Default::default()))?;
		```
		*/
		quick_load QuickLoad,
		/**
		Sends [`Quit`](Req::Quit) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.request(Req::Quit(Default::default()))?;
		```
		*/
		quit Quit,
	}
}

/// Game configuration for [`Client::create_game`] request.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GameCfg {
	/// Map for the game. Can be specified as a path or data bytes.
	pub map: Handle,
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
impl From<GameCfg> for RequestCreateGame {
	fn from(cfg: GameCfg) -> Self {
		use create_game::Participant::*;
		use sc2_prost::PlayerType;

		Self {
			map: Some(sc2_prost::request_create_game::Map::LocalMap(
				match cfg.map {
					Handle::Path(path) => sc2_prost::LocalMap {
						map_path: path.into(),
						map_data: vec![],
					},
					Handle::Data(data) => sc2_prost::LocalMap {
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
		}
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
	/// or as an observer of the player with specified [`PlayerId`].
	pub join_as: join_game::JoinAs,
	/// Interface config
	pub interface: Interface,
	pub server_ports: Option<join_game::Ports>,
	pub client_ports: Vec<join_game::Ports>,
	pub name: String,
	pub host: Option<IpAddr>,
}
impl From<JoinCfg> for RequestJoinGame {
	fn from(cfg: JoinCfg) -> Self {
		use join_game::JoinAs;
		use sc2_prost::request_join_game::Participation::*;

		Self {
			participation: Some(match cfg.join_as {
				JoinAs::Player(race) => Race(race as i32),
				JoinAs::Observer(id) => ObservedPlayerId(id.into()),
			}),
			options: Some(cfg.interface.into()),
			server_ports: cfg.server_ports.map(Into::into),
			client_ports: cfg.client_ports.into_iter().map(Into::into).collect(),
			shared_port: 0,
			player_name: cfg.name,
			host_ip: cfg.host.map_or_else(String::new, |ip| ip.to_string()),
		}
	}
}

pub mod join_game {
	use super::*;

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub enum JoinAs {
		Player(Race),
		Observer(PlayerId),
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
	pub replay: Handle,
	pub map_data: Vec<u8>,
	pub player: u32,
	pub interface: Interface,
	pub disable_fog: bool,
	pub realtime: bool,
	pub record: bool,
}
impl From<ReplayCfg> for RequestStartReplay {
	fn from(cfg: ReplayCfg) -> Self {
		use sc2_prost::request_start_replay::Replay::*;
		Self {
			replay: Some(match cfg.replay {
				Handle::Path(path) => ReplayPath(path.into()),
				Handle::Data(data) => ReplayData(data),
			}),
			map_data: cfg.map_data,
			observed_player_id: cfg.player,
			options: Some(cfg.interface.into()),
			disable_fog: cfg.disable_fog,
			realtime: cfg.realtime,
			record_replay: cfg.record,
		}
	}
}
