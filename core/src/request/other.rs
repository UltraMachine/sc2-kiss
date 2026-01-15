use super::*;
use sc2_prost::{RequestDebug, request_replay_info::Replay};

pub fn replay_info() -> ReplayInfo {
	Default::default()
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ReplayInfo(sc2_prost::RequestReplayInfo);
impl ReplayInfo {
	pub fn replay(mut self, path: Utf8PathBuf) -> Self {
		self.0.replay = Some(Replay::ReplayPath(path.into()));
		self
	}
	pub fn replay_data(mut self, data: Vec<u8>) -> Self {
		self.0.replay = Some(Replay::ReplayData(data));
		self
	}

	pub fn download_data(mut self, value: bool) -> Self {
		self.0.download_data = value;
		self
	}
}
impl From<ReplayInfo> for Request {
	fn from(r: ReplayInfo) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::ReplayInfo(r.0)),
		}
	}
}
impl ParseResponse for ReplayInfo {
	type Output = Res<sc2_prost::ResponseReplayInfo>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::ReplayInfo)?.try_map(|res| match res {
			ResponseVar::ReplayInfo(res) => {
				if res.error == 0 {
					return Ok(res);
				}
				Err(Sc2Error {
					kind: Kind::ReplayInfo,
					code: res.error,
					err: format!("{:?}", res.error()),
					desc: res.error_details,
				}
				.into())
			}
			_ => Err(BadResError(Kind::ReplayInfo, res.kind()).into()),
		})
	}
}
impl KindOf for ReplayInfo {
	fn kind(&self) -> Kind {
		Kind::ReplayInfo
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AvailableMaps;
impl From<AvailableMaps> for Request {
	fn from(_: AvailableMaps) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::AvailableMaps(Default::default())),
		}
	}
}
impl ParseResponse for AvailableMaps {
	type Output = Res<sc2_prost::ResponseAvailableMaps>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::AvailableMaps)?.try_map(|res| match res {
			ResponseVar::AvailableMaps(res) => Ok(res),
			_ => Err(BadResError(Kind::AvailableMaps, res.kind()).into()),
		})
	}
}
impl KindOf for AvailableMaps {
	fn kind(&self) -> Kind {
		Kind::AvailableMaps
	}
}

pub fn save_map() -> SaveMap {
	Default::default()
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SaveMap(sc2_prost::RequestSaveMap);
impl SaveMap {
	pub fn path(mut self, path: Utf8PathBuf) -> Self {
		self.0.map_path = path.into();
		self
	}
	pub fn data(mut self, data: Vec<u8>) -> Self {
		self.0.map_data = data;
		self
	}
}
impl From<SaveMap> for Request {
	fn from(r: SaveMap) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::SaveMap(r.0)),
		}
	}
}
impl ParseResponse for SaveMap {
	type Output = Res<()>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::SaveMap)?.try_map(|res| match res {
			ResponseVar::SaveMap(res) => {
				if res.error == 0 {
					return Ok(());
				}
				Err(Sc2Error {
					kind: Kind::SaveMap,
					code: res.error,
					err: format!("{:?}", res.error()),
					desc: String::new(),
				}
				.into())
			}
			_ => Err(BadResError(Kind::SaveMap, res.kind()).into()),
		})
	}
}
impl KindOf for SaveMap {
	fn kind(&self) -> Kind {
		Kind::SaveMap
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ping;
impl From<Ping> for Request {
	fn from(_: Ping) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::Ping(sc2_prost::RequestPing {})),
		}
	}
}
impl ParseResponse for Ping {
	type Output = Res<sc2_prost::ResponsePing>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::Ping)?.try_map(|res| match res {
			ResponseVar::Ping(res) => Ok(res),
			_ => Err(BadResError(Kind::Ping, res.kind()).into()),
		})
	}
}
impl KindOf for Ping {
	fn kind(&self) -> Kind {
		Kind::Ping
	}
}

pub fn debug(debug: Vec<sc2_prost::DebugCommand>) -> Debug {
	Debug(RequestDebug { debug })
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Debug(RequestDebug);
impl From<Debug> for Request {
	fn from(r: Debug) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::Debug(r.0)),
		}
	}
}
impl ParseResponse for Debug {
	type Output = Res<()>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::Debug)?.try_map(|res| match res {
			ResponseVar::Debug(_) => Ok(()),
			_ => Err(BadResError(Kind::Debug, res.kind()).into()),
		})
	}
}
impl KindOf for Debug {
	fn kind(&self) -> Kind {
		Kind::Debug
	}
}
