use super::*;
use bitflags::bitflags;
use camino::Utf8PathBuf;
use sc2_prost::Race;
use std::net::IpAddr;

pub mod setup;
pub use setup::*;

pub mod game;
pub use game::*;

pub mod other;
pub use other::*;

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
