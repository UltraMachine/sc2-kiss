use super::*;

/// During game
impl Client {
	/**
	Sends [`GameInfo`](Req::GameInfo) request to the server.
	Returns static data about the current game and map in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::GameInfo(Default::default()))?;
	let ResVar::GameInfo(data) = res.data else { unreachable!() };
	```
	*/
	pub fn game_info(&mut self) -> Result<Res<sc2_prost::ResponseGameInfo>> {
		unwrap_data!(self.send(Req::GameInfo(<_>::default())); GameInfo)
	}
	/**
	Sends [`Observation`](Req::Observation) request to the server.
	Returns snapshot of the current game state in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestObservation { /* Observation options */ };
	let res = client.send(Req::Observation(req))?;
	let ResVar::Observation(data) = res.data else { unreachable!() };
	```
	*/
	pub fn observation(
		&mut self,
		cfg: sc2_prost::RequestObservation,
	) -> Result<Res<sc2_prost::ResponseObservation>> {
		unwrap_data!(self.send(Req::Observation(cfg)); Observation)
	}
	/**
	Sends [`Action`](Req::Action) request to the server.
	Returns action results in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestAction { actions: vec![] };
	let res = client.send(Req::Action(req))?;
	let ResVar::Action(data) = res.data else { unreachable!() };
	let action_results = data.result;
	```
	*/
	pub fn action(
		&mut self,
		acts: Vec<sc2_prost::Action>,
	) -> Result<Res<Vec<sc2_prost::ActionResult>>> {
		let res = self.send(Req::Action(sc2_prost::RequestAction { actions: acts }));
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
	let res = client.send(Req::ObsAction(req))?;
	```
	*/
	pub fn obs_action(&mut self, acts: Vec<sc2_prost::observer_action::Action>) -> Result<Res<()>> {
		self.send(Req::ObsAction(sc2_prost::RequestObserverAction {
			actions: acts
				.into_iter()
				.map(|act| sc2_prost::ObserverAction { action: Some(act) })
				.collect(),
		}))
		.map(empty_res)
	}
	/**
	Sends [`Step`](Req::Step) request to the server.
	Returns [`simulation_loop`] in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::Step(sc2_prost::RequestStep { count: 2 }))?;
	let ResVar::Step(data) = res.data else { unreachable!() };
	let simulation_loop = data.simulation_loop;
	```

	[`simulation_loop`]: sc2_prost::ResponseStep::simulation_loop
	*/
	pub fn step(&mut self, count: u32) -> Result<Res<u32>> {
		let res = self.send(Req::Step(sc2_prost::RequestStep { count }));
		unwrap_data!(res; Step simulation_loop)
	}
	/**
	Sends [`Data`](Req::Data) request to the server.
	Returns Ids data in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestData { /* Data options */ };
	let res = client.send(Req::Data(req))?;
	let ResVar::Data(data) = res.data else { unreachable!() };
	```
	# Examples
	```no_run
	use sc2_core::request::DataFlags;

	let res = client.data(DataFlags::all())?;
	```
	*/
	pub fn data(&mut self, flags: DataFlags) -> Result<Res<sc2_prost::ResponseData>> {
		unwrap_data!(self.send(flags.into()); Data)
	}
	/**
	Sends [`Query`](Req::Query) request to the server.
	Returns query results in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestQuery { /* Query fields */ };
	let res = client.send(Req::Query(req))?;
	let ResVar::Query(data) = res.data else { unreachable!() };
	```
	*/
	pub fn query(&mut self, cfg: sc2_prost::RequestQuery) -> Result<Res<sc2_prost::ResponseQuery>> {
		unwrap_data!(self.send(Req::Query(cfg)); Query)
	}
	/**
	Sends [`SaveReplay`](Req::SaveReplay) request to the server.
	Returns replay data in response.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let res = client.send(Req::SaveReplay(Default::default()))?;
	let ResVar::SaveReplay(data) = res.data else { unreachable!() };
	```
	*/
	pub fn save_replay(&mut self) -> Result<Res<Vec<u8>>> {
		unwrap_data!(self.send(Req::SaveReplay(<_>::default())); SaveReplay data)
	}
	/**
	Sends [`MapCommand`](Req::MapCommand) request to the server.

	Convenience method for:
	```no_run
	use sc2_core::Req;

	let req = sc2_prost::RequestMapCommand { trigger_cmd: "Some map command".into() };
	let res = client.send(Req::MapCommand(req))?;
	```
	*/
	pub fn map_command(&mut self, cmd: String) -> Result<Res<()>> {
		self.send(Req::MapCommand(sc2_prost::RequestMapCommand {
			trigger_cmd: cmd,
		}))
		.map(empty_res)
	}
}

bitflags! {
	#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
	#[repr(transparent)]
	pub struct DataFlags: u8 {
		const ABILITIES = 1;
		const UNITS     = 1 << 1;
		const UPGRADES  = 1 << 2;
		const BUFFS     = 1 << 3;
		const EFFECTS   = 1 << 4;
	}
}
impl From<DataFlags> for Req {
	fn from(flags: DataFlags) -> Req {
		Req::Data(sc2_prost::RequestData {
			ability_id: flags.contains(DataFlags::ABILITIES),
			unit_type_id: flags.contains(DataFlags::UNITS),
			upgrade_id: flags.contains(DataFlags::UPGRADES),
			buff_id: flags.contains(DataFlags::BUFFS),
			effect_id: flags.contains(DataFlags::EFFECTS),
		})
	}
}
