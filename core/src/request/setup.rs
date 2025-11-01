use super::*;
use sc2_prost::{
	InterfaceOptions, LocalMap, PlayerSetup, PlayerType, PortSet, Race, SpatialCameraSetup,
	request_create_game::Map, request_join_game::Participation, request_start_replay::Replay,
};
use std::net::IpAddr;

pub fn create_game() -> CreateGame {
	Default::default()
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CreateGame(sc2_prost::RequestCreateGame);
impl CreateGame {
	pub fn map(mut self, path: Utf8PathBuf) -> Self {
		if let Some(Map::LocalMap(LocalMap { map_path, .. })) = &mut self.0.map {
			*map_path = path.into();
			return self;
		}
		self.0.map = Some(Map::LocalMap(LocalMap {
			map_path: path.into(),
			map_data: vec![],
		}));
		self
	}
	pub fn map_data(mut self, data: Vec<u8>) -> Self {
		if let Some(Map::LocalMap(LocalMap { map_data, .. })) = &mut self.0.map {
			*map_data = data;
			return self;
		}
		self.0.map = Some(Map::LocalMap(LocalMap {
			map_path: String::new(),
			map_data: data,
		}));
		self
	}
	pub fn battlenet_map(mut self, name: String) -> Self {
		if let Some(Map::BattlenetMapName(map_name)) = &mut self.0.map {
			*map_name = name;
			return self;
		}
		self.0.map = Some(Map::BattlenetMapName(name));
		self
	}

	pub fn player_setup(mut self, setup: Vec<PlayerSetup>) -> Self {
		self.0.player_setup = setup;
		self
	}

	pub fn disable_fog(mut self, value: bool) -> Self {
		self.0.disable_fog = value;
		self
	}
	pub fn random_seed(mut self, value: u32) -> Self {
		self.0.random_seed = value;
		self
	}
	pub fn realtime(mut self, value: bool) -> Self {
		self.0.realtime = value;
		self
	}
}
impl From<CreateGame> for Request {
	fn from(r: CreateGame) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::CreateGame(r.0)),
		}
	}
}
impl MapResponse for CreateGame {
	type Data = sc2_prost::ResponseCreateGame;

	fn map_res(res: ResponseVar) -> Result<Self::Data> {
		match res {
			ResponseVar::CreateGame(res) => Ok(res),
			_ => Err(BadResError(Kind::CreateGame, res.kind()).into()),
		}
	}
}
impl KindOf for CreateGame {
	fn kind(&self) -> Kind {
		Kind::CreateGame
	}
}

pub const PARTICIPANT: PlayerSetup = PlayerSetup {
	r#type: PlayerType::Participant as i32,
	race: 0,
	difficulty: 0,
	player_name: String::new(),
	ai_build: 0,
};
pub const OBSERVER: PlayerSetup = PlayerSetup {
	r#type: PlayerType::Observer as i32,
	race: 0,
	difficulty: 0,
	player_name: String::new(),
	ai_build: 0,
};

pub fn computer() -> Computer {
	Computer(PlayerSetup {
		r#type: PlayerType::Computer as i32,
		race: 0,
		difficulty: 0,
		player_name: String::new(),
		ai_build: 0,
	})
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Computer(PlayerSetup);
impl Computer {
	pub fn setup(self) -> PlayerSetup {
		self.0
	}

	pub fn race(mut self, race: Race) -> Self {
		self.0.set_race(race);
		self
	}
	pub fn difficulty(mut self, difficulty: sc2_prost::Difficulty) -> Self {
		self.0.set_difficulty(difficulty);
		self
	}
	pub fn name(mut self, name: String) -> Self {
		self.0.player_name = name;
		self
	}
	pub fn ai_build(mut self, ai_build: sc2_prost::AiBuild) -> Self {
		self.0.set_ai_build(ai_build);
		self
	}
}
impl Default for Computer {
	fn default() -> Self {
		computer()
	}
}
impl From<Computer> for PlayerSetup {
	fn from(c: Computer) -> Self {
		c.0
	}
}

pub fn join_game() -> JoinGame {
	Default::default()
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct JoinGame(sc2_prost::RequestJoinGame);
impl JoinGame {
	pub fn participant(mut self, race: Race) -> Self {
		self.0.participation = Some(Participation::Race(race as i32));
		self
	}
	pub fn observer(mut self, observed_player: PlayerId) -> Self {
		self.0.participation = Some(Participation::ObservedPlayerId(observed_player.0));
		self
	}

	pub fn interface(mut self, interface: impl Into<InterfaceOptions>) -> Self {
		self.0.options = Some(interface.into());
		self
	}

	pub fn server_ports(mut self, ports: impl Into<PortSet>) -> Self {
		self.0.server_ports = Some(ports.into());
		self
	}
	pub fn client_ports(mut self, ports: Vec<PortSet>) -> Self {
		self.0.client_ports = ports;
		self
	}

	pub fn name(mut self, name: String) -> Self {
		self.0.player_name = name;
		self
	}
	pub fn host_ip(mut self, ip: IpAddr) -> Self {
		self.0.host_ip = ip.to_string();
		self
	}
}
impl From<JoinGame> for Request {
	fn from(r: JoinGame) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::JoinGame(r.0)),
		}
	}
}
impl MapResponse for JoinGame {
	type Data = sc2_prost::ResponseJoinGame;

	fn map_res(res: ResponseVar) -> Result<Self::Data> {
		match res {
			ResponseVar::JoinGame(res) => Ok(res),
			_ => Err(BadResError(Kind::JoinGame, res.kind()).into()),
		}
	}
}
impl KindOf for JoinGame {
	fn kind(&self) -> Kind {
		Kind::JoinGame
	}
}

pub fn interface() -> Interface {
	Default::default()
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Interface(InterfaceOptions);
impl Interface {
	pub fn raw(mut self, value: bool) -> Self {
		self.0.raw = value;
		self
	}
	pub fn score(mut self, value: bool) -> Self {
		self.0.score = value;
		self
	}
	pub fn cloaked(mut self, value: bool) -> Self {
		self.0.show_cloaked = value;
		self
	}
	pub fn burrowed(mut self, value: bool) -> Self {
		self.0.show_burrowed_shadows = value;
		self
	}
	pub fn placeholders(mut self, value: bool) -> Self {
		self.0.show_placeholders = value;
		self
	}
	pub fn affect_selection(mut self, value: bool) -> Self {
		self.0.raw_affects_selection = value;
		self
	}
	pub fn crop_raw(mut self, value: bool) -> Self {
		self.0.raw_crop_to_playable_area = value;
		self
	}

	pub fn feature(mut self, camera: SpatialCameraSetup) -> Self {
		self.0.feature_layer = Some(camera);
		self
	}
	pub fn render(mut self, camera: SpatialCameraSetup) -> Self {
		self.0.render = Some(camera);
		self
	}
}
impl From<Interface> for InterfaceOptions {
	fn from(i: Interface) -> Self {
		i.0
	}
}

pub fn ports(game: u16, base: u16) -> PortSet {
	(game, base).into()
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StartReplay(sc2_prost::RequestStartReplay);
impl StartReplay {
	pub fn replay(mut self, path: Utf8PathBuf) -> Self {
		self.0.replay = Some(Replay::ReplayPath(path.into()));
		self
	}
	pub fn replay_data(mut self, data: Vec<u8>) -> Self {
		self.0.replay = Some(Replay::ReplayData(data));
		self
	}

	pub fn map_data(mut self, data: Vec<u8>) -> Self {
		self.0.map_data = data;
		self
	}
	pub fn observed_player(mut self, player: PlayerId) -> Self {
		self.0.observed_player_id = player.0;
		self
	}
	pub fn interface(mut self, interface: impl Into<InterfaceOptions>) -> Self {
		self.0.options = Some(interface.into());
		self
	}

	pub fn disable_fog(mut self, value: bool) -> Self {
		self.0.disable_fog = value;
		self
	}
	pub fn realtime(mut self, value: bool) -> Self {
		self.0.realtime = value;
		self
	}
	pub fn record_replay(mut self, value: bool) -> Self {
		self.0.record_replay = value;
		self
	}
}
impl From<StartReplay> for Request {
	fn from(r: StartReplay) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::StartReplay(r.0)),
		}
	}
}
impl MapResponse for StartReplay {
	type Data = sc2_prost::ResponseStartReplay;

	fn map_res(res: ResponseVar) -> Result<Self::Data> {
		match res {
			ResponseVar::StartReplay(res) => Ok(res),
			_ => Err(BadResError(Kind::StartReplay, res.kind()).into()),
		}
	}
}
impl KindOf for StartReplay {
	fn kind(&self) -> Kind {
		Kind::StartReplay
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RestartGame;
impl From<RestartGame> for Request {
	fn from(_: RestartGame) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::RestartGame(Default::default())),
		}
	}
}
impl MapResponse for RestartGame {
	type Data = sc2_prost::ResponseRestartGame;

	fn map_res(res: ResponseVar) -> Result<Self::Data> {
		match res {
			ResponseVar::RestartGame(res) => Ok(res),
			_ => Err(BadResError(Kind::RestartGame, res.kind()).into()),
		}
	}
}
impl KindOf for RestartGame {
	fn kind(&self) -> Kind {
		Kind::RestartGame
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LeaveGame;
impl From<LeaveGame> for Request {
	fn from(_: LeaveGame) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::LeaveGame(Default::default())),
		}
	}
}
impl MapResponse for LeaveGame {
	type Data = ();

	fn map_res(res: ResponseVar) -> Result<Self::Data> {
		match res {
			ResponseVar::LeaveGame(_) => Ok(()),
			_ => Err(BadResError(Kind::LeaveGame, res.kind()).into()),
		}
	}
}
impl KindOf for LeaveGame {
	fn kind(&self) -> Kind {
		Kind::LeaveGame
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuickSave;
impl From<QuickSave> for Request {
	fn from(_: QuickSave) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::QuickSave(Default::default())),
		}
	}
}
impl MapResponse for QuickSave {
	type Data = ();

	fn map_res(res: ResponseVar) -> Result<Self::Data> {
		match res {
			ResponseVar::QuickSave(_) => Ok(()),
			_ => Err(BadResError(Kind::QuickSave, res.kind()).into()),
		}
	}
}
impl KindOf for QuickSave {
	fn kind(&self) -> Kind {
		Kind::QuickSave
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuickLoad;
impl From<QuickLoad> for Request {
	fn from(_: QuickLoad) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::QuickLoad(Default::default())),
		}
	}
}
impl MapResponse for QuickLoad {
	type Data = ();

	fn map_res(res: ResponseVar) -> Result<Self::Data> {
		match res {
			ResponseVar::QuickLoad(_) => Ok(()),
			_ => Err(BadResError(Kind::QuickLoad, res.kind()).into()),
		}
	}
}
impl KindOf for QuickLoad {
	fn kind(&self) -> Kind {
		Kind::QuickLoad
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quit;
impl From<Quit> for Request {
	fn from(_: Quit) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::Quit(Default::default())),
		}
	}
}
impl MapResponse for Quit {
	type Data = ();

	fn map_res(res: ResponseVar) -> Result<Self::Data> {
		match res {
			ResponseVar::Quit(_) => Ok(()),
			_ => Err(BadResError(Kind::Quit, res.kind()).into()),
		}
	}
}
impl KindOf for Quit {
	fn kind(&self) -> Kind {
		Kind::Quit
	}
}
