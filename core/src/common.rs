use super::*;

/// Enum to identify kind of request/response
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
	None = 0,
	CreateGame = 1,
	JoinGame = 2,
	RestartGame = 3,
	StartReplay = 4,
	LeaveGame = 5,
	QuickSave = 6,
	QuickLoad = 7,
	Quit = 8,
	GameInfo = 9,
	Observation = 10,
	Action = 11,
	ObsAction = 21,
	Step = 12,
	Data = 13,
	Query = 14,
	SaveReplay = 15,
	ReplayInfo = 22,
	AvailableMaps = 16,
	SaveMap = 17,
	MapCommand = 18,
	Ping = 19,
	Debug = 20,
}

/// Extension trait to quickly get kind of request/response
pub trait KindOf {
	fn kind(&self) -> Kind;
}

macro_rules! match_kind {
	($e:expr) => {
		match $e {
			CreateGame(_) => Kind::CreateGame,
			JoinGame(_) => Kind::JoinGame,
			RestartGame(_) => Kind::RestartGame,
			StartReplay(_) => Kind::StartReplay,
			LeaveGame(_) => Kind::LeaveGame,
			QuickSave(_) => Kind::QuickSave,
			QuickLoad(_) => Kind::QuickLoad,
			Quit(_) => Kind::Quit,
			GameInfo(_) => Kind::GameInfo,
			Observation(_) => Kind::Observation,
			Action(_) => Kind::Action,
			ObsAction(_) => Kind::ObsAction,
			Step(_) => Kind::Step,
			Data(_) => Kind::Data,
			Query(_) => Kind::Query,
			SaveReplay(_) => Kind::SaveReplay,
			ReplayInfo(_) => Kind::ReplayInfo,
			AvailableMaps(_) => Kind::AvailableMaps,
			SaveMap(_) => Kind::SaveMap,
			MapCommand(_) => Kind::MapCommand,
			Ping(_) => Kind::Ping,
			Debug(_) => Kind::Debug,
		}
	};
}

impl KindOf for Req {
	fn kind(&self) -> Kind {
		use Req::*;
		match_kind!(self)
	}
}
impl KindOf for sc2_prost::Request {
	fn kind(&self) -> Kind {
		use Req::*;
		self.request
			.as_ref()
			.map_or(Kind::None, |req| match_kind!(req))
	}
}
impl KindOf for sc2_prost::Response {
	fn kind(&self) -> Kind {
		use ResVar::*;
		self.response
			.as_ref()
			.map_or(Kind::None, |res| match_kind!(res))
	}
}

pub(crate) fn check_status(res: Kind, now: Status, before: Status) -> Result {
	use Status::*;
	let expect = match res {
		Kind::CreateGame => vec![InitGame],
		Kind::JoinGame => vec![InGame],
		Kind::RestartGame => vec![InGame],
		Kind::StartReplay => vec![InReplay],
		Kind::LeaveGame => vec![Launched],
		Kind::Quit => vec![Unset],
		Kind::Step => vec![InGame, Ended],
		Kind::Debug => return Ok(()),
		_ => vec![before],
	};
	if expect.contains(&now) {
		Ok(())
	} else {
		Err(Error::BadStatus(now, expect))
	}
}

/// Response returned for all requests
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Res<R = ResVar> {
	pub data: R,
	pub status: Status,
	pub warns: Vec<String>,
}

pub(crate) fn check_errors(res: sc2_prost::Response, status: Status) -> Result<Res> {
	let Some(data) = res.response else {
		return Err(Error::Sc2 {
			kind: Kind::None,
			err: "Empty Response".to_owned(),
			desc: res.error.join("\n"),
		});
	};
	let warns = res.error;

	macro_rules! match_errors {
		($($Var:ident $mod:ident $($ed:expr)?),+ $(,)?) => {
			match data {
				$($Var(res) => {
					let err = res.error();
					if err == sc2_prost::$mod::Error::Unset {
						Ok(Res {
							data: $Var(res),
							status,
							warns,
						})
					} else {
						Err(Error::Sc2 {
							kind: Kind::$Var,
							err: format!("{err:?}"),
							desc: match_errors!(@res $($ed)?),
						})
					}
				})+
				_ => Ok(Res { data, status, warns }),
			}
		};
		(@$res:ident) => { $res.error_details };
		(@$res:ident $ed:expr) => { $ed };
	}
	use ResVar::*;
	match_errors! {
		CreateGame response_create_game,
		JoinGame response_join_game,
		RestartGame response_restart_game,
		StartReplay response_start_replay,
		ReplayInfo response_replay_info,
		SaveMap response_save_map String::new(),
		MapCommand response_map_command,
	}
}
