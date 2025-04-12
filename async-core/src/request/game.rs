use super::*;

pub use sc2_core::request::game::*;

/// During game
impl Client {
	/**
	Sends [`GameInfo`](Req::GameInfo) request to the server.
	Returns static data about the current game and map in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::GameInfo(Default::default())).await?;
	let ResVar::GameInfo(data) = res.data else { unreachable!() };
	```
	*/
	pub async fn game_info(&mut self) -> Result<Res<sc2_prost::ResponseGameInfo>> {
		request!(self.GameInfo)
	}
	/**
	Sends [`Observation`](Req::Observation) request to the server.
	Returns snapshot of the current game state in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestObservation { /* Observation options */ };
	let res = client.send(Req::Observation(req)).await?;
	let ResVar::Observation(data) = res.data else { unreachable!() };
	```
	*/
	pub async fn observation(
		&mut self,
		cfg: sc2_prost::RequestObservation,
	) -> Result<Res<sc2_prost::ResponseObservation>> {
		request!(self.Observation(cfg))
	}
	/**
	Sends [`Action`](Req::Action) request to the server.
	Returns action results in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestAction { actions: vec![] };
	let res = client.send(Req::Action(req)).await?;
	let ResVar::Action(data) = res.data else { unreachable!() };
	let action_results = data.result;
	```
	*/
	pub async fn action(
		&mut self,
		actions: Vec<sc2_prost::Action>,
	) -> Result<Res<Vec<sc2_prost::ActionResult>>> {
		let req = sc2_prost::RequestAction { actions };
		request!(self.Action(req)).map_res(|res| {
			res.result
				.into_iter()
				.map(|num| num.try_into().unwrap_or_default())
				.collect()
		})
	}
	/**
	Sends [`ObsAction`](Req::ObsAction) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestObserverAction { actions: vec![] };
	let res = client.send(Req::ObsAction(req)).await?;
	```
	*/
	pub async fn obs_action(
		&mut self,
		acts: Vec<sc2_prost::observer_action::Action>,
	) -> Result<Res<()>> {
		let req = sc2_prost::RequestObserverAction {
			actions: acts
				.into_iter()
				.map(|act| sc2_prost::ObserverAction { action: Some(act) })
				.collect(),
		};
		request!(self.ObsAction(req)).map(empty_res)
	}
	/**
	Sends [`Step`](Req::Step) request to the server.
	Returns [`simulation_loop`] in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::Step(sc2_prost::RequestStep { count: 2 })).await?;
	let ResVar::Step(data) = res.data else { unreachable!() };
	let simulation_loop = data.simulation_loop;
	```

	[`simulation_loop`]: sc2_prost::ResponseStep::simulation_loop
	*/
	pub async fn step(&mut self, count: u32) -> Result<Res<u32>> {
		let req = sc2_prost::RequestStep { count };
		request!(self.Step(req).simulation_loop)
	}
	/**
	Sends [`Data`](Req::Data) request to the server.
	Returns Ids data in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestData { /* Data options */ };
	let res = client.send(Req::Data(req)).await?;
	let ResVar::Data(data) = res.data else { unreachable!() };
	```
	# Examples
	```no_run
	use sc2_core::request::DataFlags;

	let res = client.data(DataFlags::all()).await?;
	```
	*/
	pub async fn data(&mut self, flags: DataFlags) -> Result<Res<sc2_prost::ResponseData>> {
		request!(self.Data(flags.into()))
	}
	/**
	Sends [`Query`](Req::Query) request to the server.
	Returns query results in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestQuery { /* Query fields */ };
	let res = client.send(Req::Query(req)).await?;
	let ResVar::Query(data) = res.data else { unreachable!() };
	```
	*/
	pub async fn query(
		&mut self,
		cfg: sc2_prost::RequestQuery,
	) -> Result<Res<sc2_prost::ResponseQuery>> {
		request!(self.Query(cfg))
	}
	/**
	Sends [`SaveReplay`](Req::SaveReplay) request to the server.
	Returns replay data in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::SaveReplay(Default::default())).await?;
	let ResVar::SaveReplay(data) = res.data else { unreachable!() };
	```
	*/
	pub async fn save_replay(&mut self) -> Result<Res<Vec<u8>>> {
		request!(self.SaveReplay.data)
	}
	/**
	Sends [`MapCommand`](Req::MapCommand) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestMapCommand { trigger_cmd: "Some map command".into() };
	let res = client.send(Req::MapCommand(req)).await?;
	```
	*/
	pub async fn map_command(&mut self, cmd: String) -> Result<Res<()>> {
		let req = sc2_prost::RequestMapCommand { trigger_cmd: cmd };
		request!(self.MapCommand(req)).map(empty_res)
	}
}
