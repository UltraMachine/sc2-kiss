use super::*;
use sc2_prost::{RequestReplayInfo, RequestSaveMap};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ReplayInfoCfg {
	pub replay: Handle,
	pub download_data: bool,
}
impl From<ReplayInfoCfg> for RequestReplayInfo {
	fn from(cfg: ReplayInfoCfg) -> Self {
		use sc2_prost::request_replay_info::Replay::*;
		Self {
			replay: Some(match cfg.replay {
				Handle::Path(path) => ReplayPath(path.into()),
				Handle::Data(data) => ReplayData(data),
			}),
			download_data: cfg.download_data,
		}
	}
}

/// Other requests
impl Client {
	/**
	Sends [`ReplayInfo`](Req::ReplayInfo) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestReplayInfo { /* Options */ };
	let res = client.request(Req::ReplayInfo(req))?;
	let ResVar::ReplayInfo(data) = res.data else { unreachable!() };
	```
	*/
	pub fn replay_info(
		&mut self,
		cfg: impl Into<RequestReplayInfo>,
	) -> Result<Res<sc2_prost::ResponseReplayInfo>> {
		request!(self.ReplayInfo(cfg.into()))
	}
	/**
	Sends [`AvailableMaps`](Req::AvailableMaps) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.request(Req::AvailableMaps(Default::default()))?;
	let ResVar::AvailableMaps(data) = res.data else { unreachable!() };
	```
	*/
	pub fn available_maps(&mut self) -> Result<Res<sc2_prost::ResponseAvailableMaps>> {
		request!(self.AvailableMaps)
	}
	/**
	Sends [`SaveMap`](Req::SaveMap) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestSaveMap { /* Save config */ };
	let res = client.request(Req::SaveMap(req))?;
	```
	*/
	pub fn save_map(&mut self, cfg: impl Into<RequestSaveMap>) -> Result<Res<()>> {
		request!(self.SaveMap(cfg.into())).map(empty_res)
	}
	/**
	Sends [`Ping`](Req::Ping) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.request(Req::Ping(Default::default()))?;
	let ResVar::Ping(data) = res.data else { unreachable!() };
	```
	*/
	pub fn ping(&mut self) -> Result<Res<sc2_prost::ResponsePing>> {
		request!(self.Ping)
	}
	/**
	Sends [`Debug`](Req::Debug) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestDebug { debug: vec![/* Debug commands */] };
	let res = client.request(Req::Debug(req))?;
	```
	*/
	pub fn debug<I>(&mut self, cmds: I) -> Result<Res<()>>
	where
		I: IntoIterator<Item = sc2_prost::debug_command::Command>,
	{
		let req = sc2_prost::RequestDebug {
			debug: cmds
				.into_iter()
				.map(|cmd| sc2_prost::DebugCommand { command: Some(cmd) })
				.collect(),
		};
		request!(self.Debug(req)).map(empty_res)
	}
}
