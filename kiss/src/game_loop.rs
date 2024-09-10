use sc2_core::{Client, Result, Status};
use sc2_prost::{RequestObservation as ReqObs, ResponseObservation as ResObs};
use std::ops::ControlFlow;

pub trait GameLoop {
	type Break;

	// `ControlFlow::Break` can be used here to break out of game loop early
	fn on_step(&mut self, client: &mut Client, obs: ResObs) -> Result<ControlFlow<Self::Break>>;

	fn req_obs(&self) -> ReqObs {
		<_>::default()
	}
	fn step_size(&self) -> u32 {
		1
	}

	fn run_game_loop(&mut self, client: &mut Client) -> Result<ControlFlow<Self::Break, ResObs>> {
		loop {
			let res = client.observation(self.req_obs())?;
			if res.status == Status::Ended {
				break Ok(ControlFlow::Continue(res.data));
			}
			if let ControlFlow::Break(val) = self.on_step(client, res.data)? {
				break Ok(ControlFlow::Break(val));
			}
			// todo: sometimes returns an error when game ends in realtime mode
			client.step(self.step_size())?;
		}
	}
}
