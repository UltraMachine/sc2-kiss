use std::net::{Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::process::{Child, Command, Output};
use std::{fs, io};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("OS error: {0}")]
	Os(#[from] io::Error),
	#[error("Can't locate game directory. Please set it explicitly.")]
	NoGameDir,
	#[error("No game versions found")]
	NoVersions,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderingLib {
	/// Path to `libEGL.so`
	Egl(PathBuf),
	/// Path to `libOSMesa.so`
	OsMesa(PathBuf),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisplayMode {
	Windowed,
	#[default]
	FullScreen,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Launcher {
	/// Addres that the SC2 API WebSocket server will listen on.
	///
	/// Defaults to `[::1]:5000`
	pub addr: SocketAddr,
	/// Path to the game directory.
	pub game_dir: PathBuf,
	/// Sets which version to use in `Versions` folder (e.g. `Base75689`)
	///
	/// If not set, launcher will try to automatically locate the latest version.
	pub version: PathBuf,
	/// Which binary to launch.
	///
	/// Defaults to `SC2_x64.exe` on windows and `SC2_x64` on linux.
	pub bin: PathBuf,
	/// Overrides working directory of the launched instance.
	///
	/// Defaults to [`game_dir`]`/Support64` on windows and [`game_dir`] on linux.
	///
	/// [`game_dir`]: Self::game_dir
	pub work_dir: PathBuf,
	/// Enables launched instance to log of all protocol requests/responses to stderr.
	pub verbose: bool,
	/// Sets rendering lib on linux. It has no effect on windows.
	pub rendering_lib: Option<RenderingLib>,
	/// Overrides behaviour after the instance is dropped.
	///
	/// Defaults to `Keep`
	pub on_drop: OnDrop,
	/// Configures SC2 window mode. Can be `Windowed` or `FullScreen`.
	///
	/// Defaults to `FullScreen`
	pub display_mode: DisplayMode,
}
impl Default for Launcher {
	fn default() -> Self {
		Self {
			addr: (Ipv6Addr::LOCALHOST, 5000).into(),
			game_dir: <_>::default(),
			version: <_>::default(),
			bin: <_>::default(),
			work_dir: <_>::default(),
			verbose: false,
			rendering_lib: None,
			on_drop: <_>::default(),
			display_mode: <_>::default(),
		}
	}
}
impl Launcher {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn command(&self) -> Result<Command> {
		#[cfg(windows)]
		fn locate_game_dir() -> Option<PathBuf> {
			let mut path = dirs::document_dir()?;
			path.extend(["StarCraft II", "ExecuteInfo.txt"]);

			let data = fs::read_to_string(path).ok()?;
			let mut path: PathBuf = data.get(13..)?.into();
			for _ in 0..3 {
				path.pop();
			}

			fs::metadata(&path)
				.is_ok_and(|p| p.is_dir())
				.then_some(path)
		}
		#[cfg(target_os = "linux")]
		let locate_game_dir = || None;

		fn default_game_dir() -> Option<PathBuf> {
			#[cfg(windows)]
			let path = "C:/Program Files (x86)/StarCraft II".into();
			#[cfg(target_os = "linux")]
			let path = {
				let mut path = dirs::home_dir()?;
				path.push("StarCraftII");
				path
			};

			fs::metadata(&path)
				.is_ok_and(|p| p.is_dir())
				.then_some(path)
		}

		let game_dir = (!self.game_dir.as_os_str().is_empty())
			.then(|| self.game_dir.clone())
			.or_else(locate_game_dir)
			.or_else(default_game_dir)
			.ok_or(Error::NoGameDir)?;

		let mut cmd_path = game_dir.clone();
		cmd_path.push("Versions");
		if !self.version.as_os_str().is_empty() {
			cmd_path.push(&self.version);
		} else {
			let latest_version = cmd_path
				.read_dir()?
				.filter_map(|e| e.ok().and_then(|e| e.file_name().into_string().ok()))
				.max_by_key(|s| s.get(..4).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0));
			if let Some(version) = latest_version {
				cmd_path.push(version);
			} else {
				return Err(Error::NoVersions);
			}
		}
		if !self.bin.as_os_str().is_empty() {
			cmd_path.push(&self.bin);
		} else {
			#[cfg(windows)]
			let default_bin = "SC2_x64.exe";
			#[cfg(target_os = "linux")]
			let default_bin = "SC2_x64";
			cmd_path.push(default_bin);
		}

		let mut cmd = Command::new(cmd_path);
		cmd.arg("-listen")
			.arg(self.addr.ip().to_string())
			.arg("-port")
			.arg(self.addr.port().to_string());
		if self.verbose {
			cmd.arg("-verbose");
		}
		if self.display_mode == DisplayMode::Windowed {
			cmd.arg("-displayMode").arg("0");
		}
		match &self.rendering_lib {
			None => {}
			Some(RenderingLib::Egl(path)) => {
				cmd.arg("-eglpath").arg(path);
			}
			Some(RenderingLib::OsMesa(path)) => {
				cmd.arg("-osmesapath").arg(path);
			}
		}

		if !self.work_dir.as_os_str().is_empty() {
			cmd.current_dir(&self.work_dir);
		} else {
			#[cfg(windows)]
			let default_work_dir = {
				let mut dir = game_dir;
				dir.push("Support64");
				dir
			};
			#[cfg(target_os = "linux")]
			let default_work_dir = game_dir;
			cmd.current_dir(default_work_dir);
		}

		Ok(cmd)
	}
	pub fn spawn(&self) -> Result<Instance> {
		self.command()?
			.spawn()
			.map(|child| Instance {
				child,
				addr: self.addr,
				on_drop: self.on_drop,
			})
			.map_err(Error::Os)
	}
	pub fn output(&self) -> Result<Output> {
		self.command()?.output().map_err(Error::Os)
	}
}

#[derive(Debug)]
pub struct Instance {
	pub child: Child,
	pub addr: SocketAddr,
	pub on_drop: OnDrop,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OnDrop {
	#[default]
	Keep,
	Wait,
	Kill,
}
impl Drop for Instance {
	fn drop(&mut self) {
		match self.on_drop {
			OnDrop::Keep => {}
			OnDrop::Wait => {
				let _ = self.child.wait();
			}
			OnDrop::Kill => {
				let _ = self.child.kill();
				let _ = self.child.wait();
			}
		}
	}
}
