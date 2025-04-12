use super::*;

pub use sc2_core::request::setup::*;
use sc2_prost::{RequestCreateGame, RequestJoinGame, RequestStartReplay};

macro_rules! simple_requests {
	($( $(#[$attr:meta])* $name:ident $Var:ident ),+ $(,)?) => {$(
		$(#[$attr])*
		pub async fn $name(&mut self) -> Result<Res<()>> {
			self.request(Req::$Var(Default::default())).await.map(empty_res)
		}
	)+};
}

/// Game setup
impl Client {
	/**
	Sends [`CreateGame`](Req::CreateGame) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestCreateGame { /* Game config */ };
	let res = client.send(Req::CreateGame(req)).await?;
	```

	# Examples
	Single player game vs computer:
	```no_run
	use sc2_core::request::{
		create_game::{Computer, Participant},
		GameCfg,
	};
	use sc2_prost::{AiBuild, Difficulty};

	let cfg = GameCfg {
		map: "Test.SC2Map".into(),
		participants: vec![
			Participant::Player,
			Participant::Computer(Computer {
				difficulty: Difficulty::VeryHard,
				ai_build: AiBuild::Rush,
				..Default::default()
			}),
		],
		..Default::default()
	};
	let res = client.create_game(cfg).await?;
	```
	*/
	pub async fn create_game(&mut self, cfg: impl Into<RequestCreateGame>) -> Result<Res<()>> {
		request!(self.CreateGame(cfg.into())).map(empty_res)
	}
	/**
	Sends [`JoinGame`](Req::JoinGame) request to the server.
	Returns [`player_id`] in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestJoinGame { /* Join config */ };
	let res = client.send(Req::JoinGame(req)).await?;
	let ResVar::JoinGame(data) = res.data else { unreachable!() };
	let player_id = data.player_id;
	```

	# Examples

	Join single player game:
	```no_run
	use sc2_core::request::JoinCfg;
	use sc2_prost::Race;

	let cfg = JoinCfg {
		join_as: Race::Terran.into(),
		name: "TestBot".into(),
		..Default::default()
	};
	let res = client.join_game(cfg).await?;
	println!("Our player_id: {}", res.data);
	```

	Join multi player game:
	```no_run
	use sc2_core::request::JoinCfg;
	use sc2_prost::Race;

	let cfg = JoinCfg {
		join_as: Race::Terran.into(),
		server_ports: Some((5001, 5002).into()),
		client_ports: vec![(5003, 5004).into(), (5005, 5006).into()],
		host: "127.0.0.1".parse().ok(),
		..Default::default()
	};
	let res = client.join_game(cfg).await?;
	println!("Our player_id: {}", res.data);
	```

	[`player_id`]: sc2_prost::ResponseJoinGame::player_id
	*/
	pub async fn join_game(&mut self, cfg: impl Into<RequestJoinGame>) -> Result<Res<PlayerId>> {
		request!(self.JoinGame(cfg.into()).player_id).map_res(Into::into)
	}
	/**
	Sends [`RestartGame`](Req::RestartGame) request to the server.
	Returns [`need_hard_reset`] flag in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::RestartGame(Default::default())).await?;
	let ResVar::RestartGame(data) = res.data else { unreachable!() };
	let need_hard_reset = data.need_hard_reset;
	```

	[`need_hard_reset`]: sc2_prost::ResponseRestartGame::need_hard_reset
	*/
	pub async fn restart_game(&mut self) -> Result<Res<bool>> {
		request!(self.RestartGame.need_hard_reset)
	}
	/**
	Sends [`StartReplay`](Req::StartReplay) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestStartReplay { /* Replay config */ };
	let res = client.send(Req::StartReplay(req)).await?;
	```
	*/
	pub async fn start_replay(&mut self, cfg: impl Into<RequestStartReplay>) -> Result<Res<()>> {
		request!(self.StartReplay(cfg.into())).map(empty_res)
	}
	simple_requests! {
		/**
		Sends [`LeaveGame`](Req::LeaveGame) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.send(Req::LeaveGame(Default::default())).await?;
		```
		*/
		leave_game LeaveGame,
		/**
		Sends [`QuickSave`](Req::QuickSave) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.send(Req::QuickSave(Default::default())).await?;
		```
		*/
		quick_save QuickSave,
		/**
		Sends [`QuickLoad`](Req::QuickLoad) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.send(Req::QuickLoad(Default::default())).await?;
		```
		*/
		quick_load QuickLoad,
		/**
		Sends [`Quit`](Req::Quit) request to the server.

		Convenience method for:
		```no_run
		use sc2_core::Req;

		let res = client.send(Req::Quit(Default::default())).await?;
		```
		*/
		quit Quit,
	}
}
