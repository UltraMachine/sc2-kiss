use std::ffi::OsStr;
use std::net::{Ipv6Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use std::{fs, io};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Addr(SocketAddr);
impl Default for Addr {
	fn default() -> Self {
		Self((Ipv6Addr::LOCALHOST, 5000).into())
	}
}

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

// todo: can you set both?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderingLib<'a> {
	/// Path to `libEGL.so`
	EGL(&'a Path),
	/// Path to `libOSMesa.so`
	OSMesa(&'a Path),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisplayMode {
	Windowed = 0,
	Borderless = 1,
	Fullscreen = 2,
}

pub fn launcher() -> LauncherBuilder<'static> {
	Default::default()
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct LauncherBuilder<'a> {
	addr: Addr,

	game_dir: PathBuf,
	version: Option<&'a Path>,
	executable: Option<&'a Path>,
	current_dir: Option<&'a Path>,

	data_dir: Option<&'a Path>,
	temp_dir: Option<&'a Path>,
	verbose: bool,
	rendering_lib: Option<RenderingLib<'a>>,

	data_version: Option<&'a OsStr>,
	display_mode: Option<DisplayMode>,
	extra: Vec<&'a OsStr>,
	on_drop: OnDrop,
}
impl LauncherBuilder<'_> {
	pub fn new() -> Self {
		Self::default()
	}

	/// Addres that the SC2 API WebSocket server will listen on.
	///
	/// Defaults to `[::1]:5000`
	pub fn addr(mut self, addr: SocketAddr) -> Self {
		self.addr = Addr(addr);
		self
	}
	/// Path to the game directory.
	pub fn game_dir(mut self, path: PathBuf) -> Self {
		self.game_dir = path;
		self
	}

	/// Enables launched instance to log of all protocol requests/responses to stderr.
	pub fn verbose(mut self, value: bool) -> Self {
		self.verbose = value;
		self
	}

	/// Configures SC2 window mode. Can be `Windowed`, `Borderless` or `FullScreen`.
	///
	/// If not specified, the game will read the value from `%USERPROFILE%\Documents\StarCraft II\Variables.txt`
	pub fn display_mode(mut self, display_mode: DisplayMode) -> Self {
		self.display_mode = Some(display_mode);
		self
	}

	/// Overrides behaviour after the instance is dropped.
	///
	/// Defaults to `Keep`
	pub fn on_drop(mut self, on_drop: OnDrop) -> Self {
		self.on_drop = on_drop;
		self
	}
}
impl<'a> LauncherBuilder<'a> {
	/// Sets which version to use in `Versions` folder (e.g. `Base75689`)
	///
	/// If not set, launcher will try to automatically locate the latest version.
	pub fn version(mut self, path: &'a (impl AsRef<Path> + ?Sized)) -> Self {
		self.version = Some(path.as_ref());
		self
	}
	/// Which executable to launch.
	///
	/// Defaults to `SC2_x64.exe` on windows and `SC2_x64` on linux.
	pub fn executable(mut self, path: &'a (impl AsRef<Path> + ?Sized)) -> Self {
		self.executable = Some(path.as_ref());
		self
	}
	/// Sets the working directory for the launched SC2 instance.
	///
	/// Defaults to [`game_dir`]`/Support64` on windows and [`game_dir`] on linux.
	///
	/// [`game_dir`]: Self::game_dir
	pub fn current_dir(mut self, path: &'a (impl AsRef<Path> + ?Sized)) -> Self {
		self.current_dir = Some(path.as_ref());
		self
	}

	pub fn data_dir(mut self, path: &'a (impl AsRef<Path> + ?Sized)) -> Self {
		self.data_dir = Some(path.as_ref());
		self
	}
	pub fn temp_dir(mut self, path: &'a (impl AsRef<Path> + ?Sized)) -> Self {
		self.temp_dir = Some(path.as_ref());
		self
	}

	pub fn egl_path(mut self, path: &'a (impl AsRef<Path> + ?Sized)) -> Self {
		self.rendering_lib = Some(RenderingLib::EGL(path.as_ref()));
		self
	}
	pub fn osmesa_path(mut self, path: &'a (impl AsRef<Path> + ?Sized)) -> Self {
		self.rendering_lib = Some(RenderingLib::OSMesa(path.as_ref()));
		self
	}
	/// Sets path to rendering library on linux. It has no effect on windows.
	pub fn rendering_lib(mut self, lib: RenderingLib<'a>) -> Self {
		self.rendering_lib = Some(lib);
		self
	}

	pub fn data_version(mut self, version_hash: &'a (impl AsRef<OsStr> + ?Sized)) -> Self {
		self.data_version = Some(version_hash.as_ref());
		self
	}

	/// Extra launcher arguments.
	///
	/// Anything from `%USERPROFILE%\Documents\StarCraft II\Variables.txt` should work.
	pub fn extra<A: AsRef<OsStr> + ?Sized>(mut self, key: &'a A, value: &'a A) -> Self {
		self.extra.push(key.as_ref());
		self.extra.push(value.as_ref());
		self
	}
}
impl LauncherBuilder<'_> {
	pub fn build(&self) -> Result<Launcher> {
		let game_dir = (!self.game_dir.as_os_str().is_empty())
			.then(|| self.game_dir.clone())
			.or_else(locate_game_dir)
			.or_else(default_game_dir)
			.ok_or(Error::NoGameDir)?;

		let mut cmd_path = game_dir.clone();
		cmd_path.push("Versions");

		if let Some(version) = self.version {
			cmd_path.push(version);
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

		if let Some(executable) = self.executable {
			cmd_path.push(executable);
		} else {
			#[cfg(windows)]
			let default_exe = "SC2_x64.exe";
			#[cfg(target_os = "linux")]
			let default_exe = "SC2_x64";
			cmd_path.push(default_exe);
		}

		let mut cmd = Command::new(cmd_path);
		cmd.arg("-listen")
			.arg(self.addr.0.ip().to_string())
			.arg("-port")
			.arg(self.addr.0.port().to_string());

		if let Some(data_version) = self.data_version {
			cmd.arg("-dataVersion").arg(data_version);
		}

		if self.verbose {
			cmd.arg("-verbose");
		}

		if let Some(display_mode) = self.display_mode {
			cmd.arg("-displayMode")
				.arg((display_mode as u8).to_string());
		}

		match &self.rendering_lib {
			None => {}
			Some(RenderingLib::EGL(path)) => {
				cmd.arg("-eglpath").arg(path);
			}
			Some(RenderingLib::OSMesa(path)) => {
				cmd.arg("-osmesapath").arg(path);
			}
		}

		cmd.args(&self.extra);

		if let Some(current_dir) = self.current_dir {
			cmd.current_dir(current_dir);
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

		Ok(Launcher {
			cmd,
			addr: self.addr.0,
			on_drop: self.on_drop,
		})
	}

	pub fn spawn(&mut self) -> Result<Instance> {
		self.build()?.spawn()
	}
}

#[derive(Debug)]
pub struct Launcher {
	cmd: Command,
	addr: SocketAddr,
	on_drop: OnDrop,
}
impl Launcher {
	pub fn builder() -> LauncherBuilder<'static> {
		Default::default()
	}

	pub fn spawn(&mut self) -> Result<Instance> {
		Ok(self.cmd.spawn().map(|child| Instance {
			child,
			addr: self.addr,
			on_drop: self.on_drop,
		})?)
	}
	pub fn output(&mut self) -> Result<Output> {
		Ok(self.cmd.output()?)
	}
	pub fn status(&mut self) -> Result<ExitStatus> {
		Ok(self.cmd.status()?)
	}

	pub fn command(&self) -> &Command {
		&self.cmd
	}
	pub fn addr(&self) -> SocketAddr {
		self.addr
	}

	pub fn stdin(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
		self.cmd.stdin(cfg);
		self
	}
	pub fn stdout(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
		self.cmd.stdout(cfg);
		self
	}
	pub fn stderr(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
		self.cmd.stderr(cfg);
		self
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

pub fn locate_game_dir() -> Option<PathBuf> {
	#[cfg(windows)]
	{
		let mut path = dirs::document_dir()?;
		path.extend(["StarCraft II", "ExecuteInfo.txt"]);

		let data = fs::read_to_string(path).ok()?;
		let mut path: PathBuf = data.get(13..)?.into();
		for _ in 0..3 {
			path.pop();
		}

		path.is_dir().then_some(path)
	}
	#[cfg(target_os = "linux")]
	None
}

pub fn default_game_dir() -> Option<PathBuf> {
	#[cfg(windows)]
	let path = PathBuf::from("C:/Program Files (x86)/StarCraft II");
	#[cfg(target_os = "linux")]
	let path = {
		let mut path = dirs::home_dir()?;
		path.push("StarCraftII");
		path
	};

	path.is_dir().then_some(path)
}
