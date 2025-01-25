use super::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ReplayInfoCfg {
	pub replay: LoadMap,
	pub download_data: bool,
}
impl From<ReplayInfoCfg> for Req {
	fn from(cfg: ReplayInfoCfg) -> Req {
		use sc2_prost::request_replay_info::Replay::*;
		Req::ReplayInfo(sc2_prost::RequestReplayInfo {
			replay: Some(match cfg.replay {
				LoadMap::Path(path) => ReplayPath(path.into()),
				LoadMap::Data(data) => ReplayData(data),
			}),
			download_data: cfg.download_data,
		})
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
	pub fn debug<I>(&mut self, cmds: I) -> Result<Res<()>>
	where
		I: IntoIterator<Item = sc2_prost::debug_command::Command>,
	{
		self.send(Req::Debug(sc2_prost::RequestDebug {
			debug: cmds
				.into_iter()
				.map(|cmd| sc2_prost::DebugCommand { command: Some(cmd) })
				.collect(),
		}))
		.map(empty_res)
	}
}
