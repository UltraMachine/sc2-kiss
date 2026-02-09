use std::ops::ControlFlow;

use sc2_core::{
	Client, Result,
	common::Res,
	request::{self, Observation, observation, step},
};
use sc2_prost::{Action, DebugCommand, ResponseObservation as ResObs, Status};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BotAction {
	pub actions: Vec<Action>,
	pub debug: Vec<DebugCommand>,
	pub leave_game: bool,
}
impl BotAction {
	pub fn new() -> Self {
		Default::default()
	}
	pub fn actions(mut self, actions: Vec<Action>) -> Self {
		self.actions = actions;
		self
	}
	pub fn debug(mut self, debug: Vec<DebugCommand>) -> Self {
		self.debug = debug;
		self
	}
	pub fn leave_game(mut self, value: bool) -> Self {
		self.leave_game = value;
		self
	}
}
impl From<Vec<Action>> for BotAction {
	fn from(actions: Vec<Action>) -> Self {
		Self {
			actions,
			..Default::default()
		}
	}
}
impl From<Vec<DebugCommand>> for BotAction {
	fn from(debug: Vec<DebugCommand>) -> Self {
		Self {
			debug,
			..Default::default()
		}
	}
}
impl From<(Vec<Action>, Vec<DebugCommand>)> for BotAction {
	fn from((actions, debug): (Vec<Action>, Vec<DebugCommand>)) -> Self {
		Self {
			actions,
			debug,
			..Default::default()
		}
	}
}

pub trait PlayGame {
	fn on_start(&mut self, obs: &ResObs, client: &mut Client) -> Result<BotAction>;
	fn on_step(&mut self, obs: ResObs, client: &mut Client) -> Result<BotAction>;
	fn on_end(&mut self, obs: ResObs, client: &mut Client) -> Result;

	fn step_size(&self) -> u32 {
		1
	}
	fn cheat_vision(&self) -> bool {
		false
	}
}

fn execute(action: BotAction, client: &mut Client) -> Result<ControlFlow<()>> {
	let actions = action.actions;
	if !actions.is_empty() {
		client.request(request::action(actions))?;
	}
	let debug = action.debug;
	if !debug.is_empty() {
		client.request(request::debug(debug))?;
	}
	Ok(if action.leave_game {
		ControlFlow::Break(())
	} else {
		ControlFlow::Continue(())
	})
}

fn play_step(
	bot: &mut impl PlayGame,
	res: Res<ResObs>,
	client: &mut Client,
	realtime: bool,
) -> Result<ControlFlow<()>> {
	if res.status == Status::Ended {
		bot.on_end(res.data, client)?;
		return Ok(ControlFlow::Break(()));
	}
	let action = bot.on_step(res.data, client)?;
	let cf = execute(action, client)?;

	if !realtime && cf.is_continue() {
		client.request(step(bot.step_size()))?;
	}
	Ok(cf)
}

pub fn play_game(bot: &mut impl PlayGame, client: &mut Client, realtime: bool) -> Result {
	fn req_obs(bot: &impl PlayGame) -> Observation {
		observation().disable_fog(bot.cheat_vision())
	}
	fn game_loop(res: &Res<ResObs>) -> u32 {
		res.data.observation.as_ref().map_or(0, |obs| obs.game_loop)
	}

	let res = client.request(req_obs(bot))?;
	let action = bot.on_start(&res.data, client)?;
	if execute(action, client)?.is_break() {
		return Ok(());
	}

	let mut last_loop = game_loop(&res);
	if play_step(bot, res, client, realtime)?.is_break() {
		return Ok(());
	}
	loop {
		let req_loop = if realtime {
			last_loop + bot.step_size()
		} else {
			0
		};
		let res = client.request(req_obs(bot).game_loop(req_loop))?;
		last_loop = game_loop(&res);
		if play_step(bot, res, client, realtime)?.is_break() {
			break Ok(());
		}
	}
}
