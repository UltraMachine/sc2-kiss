use super::*;
use sc2_prost::Status;
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

impl KindOf for RequestVar {
	fn kind(&self) -> Kind {
		use RequestVar::*;
		match_kind!(self)
	}
}
impl KindOf for ResponseVar {
	fn kind(&self) -> Kind {
		use ResponseVar::*;
		match_kind!(self)
	}
}
impl<R: KindOf> KindOf for Res<R> {
	fn kind(&self) -> Kind {
		self.data.kind()
	}
}
impl KindOf for Request {
	fn kind(&self) -> Kind {
		self.request.as_ref().map_or(Kind::None, KindOf::kind)
	}
}
impl KindOf for Response {
	fn kind(&self) -> Kind {
		self.response.as_ref().map_or(Kind::None, KindOf::kind)
	}
}

/// Describes some error returned in the response
#[derive(Debug, Error)]
#[error("SC2 {kind:?} Error: `{err}` {desc}")]
pub struct Sc2Error {
	pub(crate) kind: Kind,
	pub(crate) code: i32,
	pub(crate) err: String,
	pub(crate) desc: String,
}
impl KindOf for Sc2Error {
	fn kind(&self) -> Kind {
		self.kind
	}
}
impl Sc2Error {
	pub fn code(&self) -> i32 {
		self.code
	}
}

/// Response type returned for all requests
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Res<R = ResponseVar> {
	pub data: R,
	pub status: Status,
	pub warnings: Vec<String>,
	pub id: u32,
}
impl TryFrom<Response> for Res {
	type Error = Sc2Error;
	fn try_from(res: Response) -> Result<Res, Sc2Error> {
		let status = res.status();
		let Some(data) = res.response else {
			return Err(Sc2Error {
				kind: Kind::None,
				code: 0,
				err: "Empty Response".into(),
				desc: res.error.join("\n"),
			});
		};
		Ok(Res {
			data,
			status,
			warnings: res.error,
			id: res.id,
		})
	}
}
impl From<Res> for Response {
	fn from(r: Res) -> Self {
		Self {
			response: Some(r.data),
			id: r.id,
			error: r.warnings,
			status: r.status as i32,
		}
	}
}

impl<R> Res<R> {
	pub fn map<T>(self, f: impl FnOnce(R) -> T) -> Res<T> {
		Res {
			data: f(self.data),
			status: self.status,
			warnings: self.warnings,
			id: self.id,
		}
	}
	pub fn try_map<T, E>(self, f: impl FnOnce(R) -> Result<T, E>) -> Result<Res<T>, E> {
		Ok(Res {
			data: f(self.data)?,
			status: self.status,
			warnings: self.warnings,
			id: self.id,
		})
	}
}

pub trait ResultResExt<R, E> {
	fn map_res<T>(self, f: impl FnOnce(R) -> T) -> Result<Res<T>, E>;
	fn try_map_res<T>(self, f: impl FnOnce(R) -> Result<T, E>) -> Result<Res<T>, E>;
}
impl<R, E> ResultResExt<R, E> for Result<Res<R>, E> {
	fn map_res<T>(self, f: impl FnOnce(R) -> T) -> Result<Res<T>, E> {
		self.map(|res| res.map(f))
	}
	fn try_map_res<T>(self, f: impl FnOnce(R) -> Result<T, E>) -> Result<Res<T>, E> {
		self.and_then(|res| res.try_map(f))
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
		match self.0 {
			0 => write!(f, "Neutral"),
			i => write!(f, "Player {i}"),
		}
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

#[derive(Debug, Error)]
#[error("Bad response: `{0:?}`, expected `{1:?}`")]
pub struct BadResError(pub Kind, pub Kind);

pub trait ParseResponse {
	type Output;
	fn parse(res: Response) -> Result<Self::Output>;
}

impl ParseResponse for Request {
	type Output = Response;

	fn parse(res: Response) -> Result<Self::Output> {
		Ok(res)
	}
}

pub trait RequestSetId {
	fn id(self, id: u32) -> SetId<Self>
	where
		Self: Sized;
}
impl<R> RequestSetId for R {
	fn id(self, id: u32) -> SetId<Self> {
		SetId(self, id)
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetId<R>(R, u32);
impl<R: Into<Request>> From<SetId<R>> for Request {
	fn from(r: SetId<R>) -> Self {
		Self {
			id: r.1,
			..r.0.into()
		}
	}
}
impl<R: ParseResponse> ParseResponse for SetId<R> {
	type Output = R::Output;

	fn parse(res: Response) -> Result<Self::Output> {
		R::parse(res)
	}
}
impl<R: KindOf> KindOf for SetId<R> {
	fn kind(&self) -> Kind {
		self.0.kind()
	}
}

#[doc(hidden)]
pub mod internal {
	use prost::Message as _;
	use tungstenite::Message;

	use super::*;

	pub fn req_into_msg(req: Request) -> Message {
		Message::Binary(req.encode_to_vec().into())
	}
	pub fn res_from_msg(msg: Message) -> Result<Response> {
		Ok(Response::decode(msg.into_data())?)
	}
	#[cfg(feature = "server")]
	pub fn req_from_msg(msg: Message) -> server::Result<Request> {
		Ok(Request::decode(msg.into_data())?)
	}
	pub fn res_into_msg(res: Response) -> Message {
		Message::Binary(res.encode_to_vec().into())
	}

	pub fn convert_res(response: Response, kind: Kind) -> Result<Res> {
		Res::try_from(response).map_err(|mut e| {
			e.kind = kind;
			e.into()
		})
	}
}
