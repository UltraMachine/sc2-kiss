use super::*;
use sc2_prost::{Action as PbAction, ObserverAction as PbObsAction};
use sc2_prost::{RequestAction, RequestMapCommand, RequestObserverAction, RequestStep};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameInfo;
impl From<GameInfo> for Request {
	fn from(_: GameInfo) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::GameInfo(Default::default())),
		}
	}
}
impl ParseResponse for GameInfo {
	type Output = Res<sc2_prost::ResponseGameInfo>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::GameInfo)?.try_map(|res| match res {
			ResponseVar::GameInfo(res) => Ok(res),
			_ => Err(BadResError(Kind::GameInfo, res.kind()).into()),
		})
	}
}
impl KindOf for GameInfo {
	fn kind(&self) -> Kind {
		Kind::GameInfo
	}
}

pub fn observation() -> Observation {
	Default::default()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Observation(sc2_prost::RequestObservation);
impl Observation {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn disable_fog(mut self, value: bool) -> Self {
		self.0.disable_fog = value;
		self
	}
	pub fn game_loop(mut self, value: u32) -> Self {
		self.0.game_loop = value;
		self
	}
}
impl From<Observation> for Request {
	fn from(r: Observation) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::Observation(r.0)),
		}
	}
}
impl ParseResponse for Observation {
	type Output = Res<sc2_prost::ResponseObservation>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::Observation)?.try_map(|res| match res {
			ResponseVar::Observation(res) => Ok(res),
			_ => Err(BadResError(Kind::Observation, res.kind()).into()),
		})
	}
}
impl KindOf for Observation {
	fn kind(&self) -> Kind {
		Kind::Observation
	}
}

pub fn action(actions: Vec<PbAction>) -> Action {
	actions.into()
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Action(RequestAction);
impl Action {
	pub fn new() -> Self {
		Self::default()
	}
}
impl From<Vec<PbAction>> for Action {
	fn from(actions: Vec<PbAction>) -> Self {
		Self(RequestAction { actions })
	}
}
impl FromIterator<PbAction> for Action {
	fn from_iter<I: IntoIterator<Item = PbAction>>(iter: I) -> Self {
		iter.into_iter().collect::<Vec<_>>().into()
	}
}
impl Extend<PbAction> for Action {
	fn extend<I: IntoIterator<Item = PbAction>>(&mut self, iter: I) {
		self.0.actions.extend(iter);
	}
}
impl From<Action> for Request {
	fn from(r: Action) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::Action(r.0)),
		}
	}
}
impl ParseResponse for Action {
	type Output = Res<sc2_prost::ResponseAction>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::Action)?.try_map(|res| match res {
			ResponseVar::Action(res) => Ok(res),
			_ => Err(BadResError(Kind::Action, res.kind()).into()),
		})
	}
}
impl KindOf for Action {
	fn kind(&self) -> Kind {
		Kind::Action
	}
}

pub fn observer_action(actions: Vec<PbObsAction>) -> ObserverAction {
	actions.into()
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ObserverAction(RequestObserverAction);
impl ObserverAction {
	pub fn new() -> Self {
		Self::default()
	}
}
impl From<Vec<PbObsAction>> for ObserverAction {
	fn from(actions: Vec<PbObsAction>) -> Self {
		Self(RequestObserverAction { actions })
	}
}
impl FromIterator<PbObsAction> for ObserverAction {
	fn from_iter<I: IntoIterator<Item = PbObsAction>>(iter: I) -> Self {
		iter.into_iter().collect::<Vec<_>>().into()
	}
}
impl Extend<PbObsAction> for ObserverAction {
	fn extend<I: IntoIterator<Item = PbObsAction>>(&mut self, iter: I) {
		self.0.actions.extend(iter);
	}
}
impl From<ObserverAction> for Request {
	fn from(r: ObserverAction) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::ObsAction(r.0)),
		}
	}
}
impl ParseResponse for ObserverAction {
	type Output = Res<()>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::ObsAction)?.try_map(|res| match res {
			ResponseVar::ObsAction(_) => Ok(()),
			_ => Err(BadResError(Kind::ObsAction, res.kind()).into()),
		})
	}
}
impl KindOf for ObserverAction {
	fn kind(&self) -> Kind {
		Kind::ObsAction
	}
}

pub fn step(count: u32) -> Step {
	count.into()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Step(pub u32);
impl From<u32> for Step {
	fn from(count: u32) -> Self {
		Step(count)
	}
}
impl From<Step> for Request {
	fn from(r: Step) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::Step(RequestStep { count: r.0 })),
		}
	}
}
impl ParseResponse for Step {
	type Output = Res<u32>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::Step)?.try_map(|res| match res {
			ResponseVar::Step(res) => Ok(res.simulation_loop),
			_ => Err(BadResError(Kind::Step, res.kind()).into()),
		})
	}
}
impl KindOf for Step {
	fn kind(&self) -> Kind {
		Kind::Step
	}
}

pub fn data() -> Data {
	Default::default()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Data(sc2_prost::RequestData);
impl Data {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn abilities(mut self, value: bool) -> Self {
		self.0.ability_id = value;
		self
	}
	pub fn units(mut self, value: bool) -> Self {
		self.0.unit_type_id = value;
		self
	}
	pub fn upgrades(mut self, value: bool) -> Self {
		self.0.upgrade_id = value;
		self
	}
	pub fn buffs(mut self, value: bool) -> Self {
		self.0.buff_id = value;
		self
	}
	pub fn effects(mut self, value: bool) -> Self {
		self.0.effect_id = value;
		self
	}

	pub fn all(self) -> Self {
		self.abilities(true)
			.units(true)
			.upgrades(true)
			.buffs(true)
			.effects(true)
	}
}
impl From<Data> for Request {
	fn from(r: Data) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::Data(r.0)),
		}
	}
}
impl ParseResponse for Data {
	type Output = Res<sc2_prost::ResponseData>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::Data)?.try_map(|res| match res {
			ResponseVar::Data(res) => Ok(res),
			_ => Err(BadResError(Kind::Data, res.kind()).into()),
		})
	}
}
impl KindOf for Data {
	fn kind(&self) -> Kind {
		Kind::Data
	}
}

pub fn query() -> Query {
	Default::default()
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Query(sc2_prost::RequestQuery);
impl Query {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn pathing(mut self, pathing: Vec<sc2_prost::RequestQueryPathing>) -> Self {
		self.0.pathing = pathing;
		self
	}
	pub fn abilities(mut self, abilities: Vec<sc2_prost::RequestQueryAvailableAbilities>) -> Self {
		self.0.abilities = abilities;
		self
	}
	pub fn placements(mut self, placements: Vec<sc2_prost::RequestQueryBuildingPlacement>) -> Self {
		self.0.placements = placements;
		self
	}
	pub fn ignore_resource_requirements(mut self, value: bool) -> Self {
		self.0.ignore_resource_requirements = value;
		self
	}
}
impl From<Query> for Request {
	fn from(r: Query) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::Query(r.0)),
		}
	}
}
impl ParseResponse for Query {
	type Output = Res<sc2_prost::ResponseQuery>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::Query)?.try_map(|res| match res {
			ResponseVar::Query(res) => Ok(res),
			_ => Err(BadResError(Kind::Query, res.kind()).into()),
		})
	}
}
impl KindOf for Query {
	fn kind(&self) -> Kind {
		Kind::Query
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SaveReplay;
impl From<SaveReplay> for Request {
	fn from(_: SaveReplay) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::SaveReplay(Default::default())),
		}
	}
}
impl ParseResponse for SaveReplay {
	type Output = Res<Vec<u8>>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::SaveReplay)?.try_map(|res| match res {
			ResponseVar::SaveReplay(res) => Ok(res.data),
			_ => Err(BadResError(Kind::SaveReplay, res.kind()).into()),
		})
	}
}
impl KindOf for SaveReplay {
	fn kind(&self) -> Kind {
		Kind::SaveReplay
	}
}

pub fn map_command(cmd: String) -> MapCommand {
	cmd.into()
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct MapCommand(RequestMapCommand);
impl From<String> for MapCommand {
	fn from(cmd: String) -> Self {
		MapCommand(RequestMapCommand { trigger_cmd: cmd })
	}
}
impl From<MapCommand> for Request {
	fn from(r: MapCommand) -> Self {
		Self {
			id: 0,
			request: Some(RequestVar::MapCommand(r.0)),
		}
	}
}
impl ParseResponse for MapCommand {
	type Output = Res<()>;

	fn parse(res: Response) -> Result<Self::Output> {
		convert_res(res, Kind::MapCommand)?.try_map(|res| match res {
			ResponseVar::MapCommand(res) => {
				if res.error == 0 {
					return Ok(());
				}
				Err(Sc2Error {
					kind: Kind::MapCommand,
					code: res.error,
					err: format!("{:?}", res.error()),
					desc: res.error_details,
				}
				.into())
			}
			_ => Err(BadResError(Kind::MapCommand, res.kind()).into()),
		})
	}
}
impl KindOf for MapCommand {
	fn kind(&self) -> Kind {
		Kind::MapCommand
	}
}
