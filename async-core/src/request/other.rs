use super::*;
use sc2_prost::{RequestReplayInfo, RequestSaveMap};

pub use sc2_core::request::other::*;

/// Other requests
impl Client {
	/**
	Sends [`ReplayInfo`](Req::ReplayInfo) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestReplayInfo { /* Options */ };
	let res = client.send(Req::ReplayInfo(req)).await?;
	let ResVar::ReplayInfo(data) = res.data else { unreachable!() };
	```
	*/
	pub async fn replay_info(
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

	let res = client.send(Req::AvailableMaps(Default::default())).await?;
	let ResVar::AvailableMaps(data) = res.data else { unreachable!() };
	```
	*/
	pub async fn available_maps(&mut self) -> Result<Res<sc2_prost::ResponseAvailableMaps>> {
		request!(self.AvailableMaps)
	}
	/**
	Sends [`SaveMap`](Req::SaveMap) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestSaveMap { /* Save config */ };
	let res = client.send(Req::SaveMap(req)).await?;
	```
	*/
	pub async fn save_map(&mut self, cfg: impl Into<RequestSaveMap>) -> Result<Res<()>> {
		request!(self.SaveMap(cfg.into())).map(empty_res)
	}
	/**
	Sends [`Ping`](Req::Ping) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::Ping(Default::default())).await?;
	let ResVar::Ping(data) = res.data else { unreachable!() };
	```
	*/
	pub async fn ping(&mut self) -> Result<Res<sc2_prost::ResponsePing>> {
		request!(self.Ping)
	}
	/**
	Sends [`Debug`](Req::Debug) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestDebug { debug: vec![/* Debug commands */] };
	let res = client.send(Req::Debug(req)).await?;
	```
	*/
	pub async fn debug<I>(&mut self, cmds: I) -> Result<Res<()>>
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
