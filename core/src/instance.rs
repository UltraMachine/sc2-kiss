use std::io;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::process::{Child, Command, Output};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderingLib {
	/// Path to `libEGL.so`
	Egl(PathBuf),
	/// Path to `libOSMesa.so`
	OsMesa(PathBuf),
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
}
impl Default for Launcher {
	fn default() -> Self {
		Self {
			addr: SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 5000),
			game_dir: <_>::default(),
			version: <_>::default(),
			bin: <_>::default(),
			work_dir: <_>::default(),
			verbose: false,
			rendering_lib: None,
		}
	}
}
impl Launcher {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn command(&self) -> io::Result<Command> {
		let mut cmd_path = self.game_dir.clone();
		cmd_path.push("Versions");
		if !self.version.as_os_str().is_empty() {
			cmd_path.push(&self.version);
		} else {
			let latest_version = cmd_path
				.read_dir()?
				.filter_map(|e| e.ok().and_then(|e| e.file_name().into_string().ok()))
				.max_by_key(|s| s.split_at(4).1.parse::<u32>().unwrap_or(0));
			if let Some(version) = latest_version {
				cmd_path.push(version);
			} else {
				todo!("return error when can't find latest version");
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
				let mut dir = self.game_dir.clone();
				dir.push("Support64");
				dir
			};
			#[cfg(target_os = "linux")]
			let default_work_dir = &self.game_dir;
			cmd.current_dir(default_work_dir);
		}

		Ok(cmd)
	}
	pub fn spawn(&self) -> io::Result<Instance> {
		self.command()?.spawn().map(|child| Instance {
			child,
			addr: self.addr,
			on_drop: <_>::default(),
		})
	}
	pub fn output(&self) -> io::Result<Output> {
		self.command()?.output()
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
impl Instance {
	pub fn keep_on_drop(&mut self) {
		self.on_drop = OnDrop::Keep;
	}
	pub fn wait_on_drop(&mut self) {
		self.on_drop = OnDrop::Wait;
	}
	pub fn kill_on_drop(&mut self) {
		self.on_drop = OnDrop::Kill;
	}
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
