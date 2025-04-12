use super::*;
use prost::Message;
use std::fmt;

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
impl KindOf for ResVar {
	fn kind(&self) -> Kind {
		use ResVar::*;
		match_kind!(self)
	}
}
impl KindOf for Res {
	fn kind(&self) -> Kind {
		self.data.kind()
	}
}
impl KindOf for sc2_prost::Request {
	fn kind(&self) -> Kind {
		self.request.as_ref().map_or(Kind::None, KindOf::kind)
	}
}
impl KindOf for sc2_prost::Response {
	fn kind(&self) -> Kind {
		self.response.as_ref().map_or(Kind::None, KindOf::kind)
	}
}

/// Response type returned for all requests
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Res<R = ResVar> {
	pub data: R,
	pub status: Status,
	pub warns: Vec<String>,
}
impl TryFrom<sc2_prost::Response> for Res {
	type Error = Sc2Error;
	fn try_from(res: sc2_prost::Response) -> Result<Res, Sc2Error> {
		let status = res.status();
		let Some(data) = res.response else {
			return Err(Sc2Error {
				kind: Kind::None,
				err: "Empty Response".to_owned(),
				desc: res.error.join("\n"),
			});
		};
		Ok(Res {
			data,
			status,
			warns: res.error,
		})
	}
}
/// Describes some error returned in the response
#[derive(Debug, Error)]
#[error("Sc2 {kind:?} Error: `{err}`\n{desc}")]
pub struct Sc2Error {
	kind: Kind,
	err: String,
	desc: String,
}

impl<R> Res<R> {
	pub fn map<T>(self, data: impl FnOnce(R) -> T) -> Res<T> {
		Res {
			data: data(self.data),
			status: self.status,
			warns: self.warns,
		}
	}
}

pub trait ResultResExt<R> {
	fn map_res<T>(self, f: impl FnOnce(R) -> T) -> Result<Res<T>>;
}
impl<R> ResultResExt<R> for Result<Res<R>> {
	fn map_res<T>(self, f: impl FnOnce(R) -> T) -> Result<Res<T>> {
		self.map(|res| res.map(f))
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PlayerId(pub u32);
impl PlayerId {
	pub const NEUTRAL: Self = Self(0);
}
impl fmt::Display for PlayerId {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl From<u32> for PlayerId {
	fn from(num: u32) -> Self {
		Self(num)
	}
}
impl From<PlayerId> for u32 {
	fn from(player_id: PlayerId) -> u32 {
		player_id.0
	}
}

#[doc(hidden)]
pub mod internal {
	use super::*;

	pub fn check_res(res: Res, req_kind: Kind, old_status: &mut Status) -> Result<Res> {
		let kind = res.kind();
		if kind != req_kind {
			return Err(Error::BadRes(kind, req_kind));
		}
		check_errors(&res)?;
		check_status(kind, res.status, *old_status)?;
		*old_status = res.status;
		Ok(res)
	}

	fn check_status(kind: Kind, now: Status, before: Status) -> Result {
		use Status::*;
		if before == Unset {
			return Ok(());
		}
		let expect = match kind {
			Kind::CreateGame => vec![InitGame],
			Kind::JoinGame => vec![InGame],
			Kind::RestartGame => vec![InGame],
			Kind::StartReplay => vec![InReplay],
			Kind::LeaveGame => vec![Launched],
			Kind::Quit => vec![Quit],
			Kind::Step | Kind::Observation => vec![InGame, InReplay, Ended],
			Kind::Debug => return Ok(()),
			_ => vec![before],
		};
		if expect.contains(&now) {
			Ok(())
		} else {
			Err(Error::BadStatus(now, expect))
		}
	}

	fn check_errors(res: &Res) -> Result {
		macro_rules! match_errors {
			($($Var:ident $mod:ident $($ed:expr)?),+ $(,)?) => {
				match res.data {
					$(ResVar::$Var(ref var) => {
						let err = var.error();
						if err != sc2_prost::$mod::Error::Unset {
							return Err(Sc2Error {
								kind: Kind::$Var,
								err: format!("{err:?}"),
								desc: match_errors!(@var $($ed)?),
							}
							.into())
						}
					})+
					_ => {}
				}
			};
			(@$res:ident) => { $res.error_details.clone() };
			(@$res:ident $ed:expr) => { $ed };
		}
		match_errors! {
			CreateGame response_create_game,
			JoinGame response_join_game,
			RestartGame response_restart_game,
			StartReplay response_start_replay,
			ReplayInfo response_replay_info,
			SaveMap response_save_map String::new(),
			MapCommand response_map_command,
		}
		Ok(())
	}

	pub fn req_into_msg(req: Req) -> tungstenite::Message {
		let req = sc2_prost::Request {
			id: 0,
			request: Some(req),
		};
		tungstenite::Message::Binary(req.encode_to_vec().into())
	}
	pub fn res_from_msg(msg: tungstenite::Message, req_kind: Kind) -> Result<Res> {
		let res = sc2_prost::Response::decode(msg.into_data())?;
		Res::try_from(res).map_err(|mut e| {
			if e.kind == Kind::None {
				e.kind = req_kind;
			}
			e.into()
		})
	}

	pub fn empty_res<R>(res: Res<R>) -> Res<()> {
		res.map(|_| ())
	}
}
