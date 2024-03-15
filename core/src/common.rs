use super::*;
use prost::Message;

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
#[error("`{kind:?}` Error: `{err}`\n{desc}")]
pub struct Sc2Error {
	kind: Kind,
	err: String,
	desc: String,
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
		check_status(kind, res.status, old_status)?;
		Ok(res)
	}

	fn check_status(kind: Kind, now: Status, before: &mut Status) -> Result {
		use Status::*;
		let expect = match kind {
			Kind::CreateGame => vec![InitGame],
			Kind::JoinGame => vec![InGame],
			Kind::RestartGame => vec![InGame],
			Kind::StartReplay => vec![InReplay],
			Kind::LeaveGame => vec![Launched],
			Kind::Quit => vec![Unset],
			Kind::Step => vec![InGame, Ended],
			Kind::Debug => return Ok(()),
			_ => vec![*before],
		};
		if expect.contains(&now) {
			*before = now;
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
		tungstenite::Message::Binary(req.encode_to_vec())
	}
	pub fn res_from_msg(msg: tungstenite::Message) -> Result<Res> {
		Ok(sc2_prost::Response::decode(msg.into_data().as_slice())?.try_into()?)
	}
}
