use super::*;
use bitflags::bitflags;
use camino::Utf8PathBuf;
use sc2_prost::Race;
use std::net::IpAddr;

pub mod setup;
pub use setup::*;

pub mod game;
pub use game::*;

pub mod common {
	use super::*;

	#[derive(Debug, Default, Clone, PartialEq)]
	pub struct Interface {
		pub flags: Flags,
		pub feature: Option<sc2_prost::SpatialCameraSetup>,
		pub render: Option<sc2_prost::SpatialCameraSetup>,
	}
	impl From<Interface> for sc2_prost::InterfaceOptions {
		fn from(i: Interface) -> Self {
			Self {
				raw: i.flags.contains(Flags::RAW),
				score: i.flags.contains(Flags::SCORE),
				feature_layer: i.feature,
				render: i.render,
				show_cloaked: i.flags.contains(Flags::CLOAKED),
				show_burrowed_shadows: i.flags.contains(Flags::BURROWED),
				show_placeholders: i.flags.contains(Flags::PLACEHOLDERS),
				raw_affects_selection: i.flags.contains(Flags::AFFECT_SELECTION),
				raw_crop_to_playable_area: i.flags.contains(Flags::CROP_RAW),
			}
		}
	}

	bitflags! {
		#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
		#[repr(transparent)]
		pub struct Flags: u8 {
			const RAW              = 1;
			const SCORE            = 1 << 1;
			const CLOAKED          = 1 << 2;
			const BURROWED         = 1 << 3;
			const PLACEHOLDERS     = 1 << 4;
			const AFFECT_SELECTION = 1 << 5;
			const CROP_RAW         = 1 << 6;
		}
	}
	impl Default for Flags {
		fn default() -> Self {
			Self::RAW | Self::CLOAKED | Self::BURROWED | Self::AFFECT_SELECTION | Self::CROP_RAW
		}
	}

	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	pub enum Load {
		Path(Utf8PathBuf),
		Data(Vec<u8>),
	}
	impl Default for Load {
		fn default() -> Self {
			Load::Path(<_>::default())
		}
	}
	impl<T: Into<Utf8PathBuf>> From<T> for Load {
		fn from(path: T) -> Self {
			Load::Path(path.into())
		}
	}
}
pub use common::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ReplayInfoCfg {
	pub replay: common::Load,
	pub download_data: bool,
}
impl From<ReplayInfoCfg> for Req {
	fn from(cfg: ReplayInfoCfg) -> Req {
		use sc2_prost::request_replay_info::Replay::*;
		Req::ReplayInfo(sc2_prost::RequestReplayInfo {
			replay: Some(match cfg.replay {
				Load::Path(path) => ReplayPath(path.into()),
				Load::Data(data) => ReplayData(data),
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
	pub fn debug(&mut self, cmds: Vec<sc2_prost::debug_command::Command>) -> Result<Res<()>> {
		self.send(Req::Debug(sc2_prost::RequestDebug {
			debug: cmds
				.into_iter()
				.map(|cmd| sc2_prost::DebugCommand { command: Some(cmd) })
				.collect(),
		}))
		.map(empty_res)
	}
}
