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
		unwrap_data!(self.send(Req::GameInfo(<_>::default())).await; GameInfo)
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
		unwrap_data!(self.send(Req::Observation(cfg)).await; Observation)
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
		acts: Vec<sc2_prost::Action>,
	) -> Result<Res<Vec<sc2_prost::ActionResult>>> {
		let req = Req::Action(sc2_prost::RequestAction { actions: acts });
		let res = self.send(req).await;
		Ok(unwrap_data!(res; Action result)?.map(|result| {
			result
				.into_iter()
				.map(|num| num.try_into().unwrap_or_default())
				.collect()
		}))
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
		self.send(Req::ObsAction(sc2_prost::RequestObserverAction {
			actions: acts
				.into_iter()
				.map(|act| sc2_prost::ObserverAction { action: Some(act) })
				.collect(),
		}))
		.await
		.map(empty_res)
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
		let res = self.send(Req::Step(sc2_prost::RequestStep { count })).await;
		unwrap_data!(res; Step simulation_loop)
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
		unwrap_data!(self.send(flags.into()).await; Data)
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
		unwrap_data!(self.send(Req::Query(cfg)).await; Query)
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
		unwrap_data!(self.send(Req::SaveReplay(<_>::default())).await; SaveReplay data)
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
		self.send(Req::MapCommand(sc2_prost::RequestMapCommand {
			trigger_cmd: cmd,
		}))
		.await
		.map(empty_res)
	}
}
