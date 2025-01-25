use sc2_core::{Client, Result};
use sc2_prost::{action_chat::Channel, Action, ActionChat};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Chat(Vec<ActionChat>);
impl Chat {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn say(&mut self, msg: String) {
		self.0.push(ActionChat {
			channel: 0,
			message: msg,
		});
	}
	pub fn log(&mut self, msg: String) {
		self.0.push(ActionChat {
			channel: Channel::Team as i32,
			message: msg,
		});
	}

	pub fn flush(&mut self, client: &mut Client) -> Result {
		let mut buf = vec![];
		self.flush_to_vec(&mut buf);
		if buf.is_empty() {
			return Ok(());
		}
		client.action(buf).map(|_| ())
	}
	pub fn flush_to_vec(&mut self, buf: &mut Vec<Action>) {
		buf.extend(self.0.drain(..).map(|a| Action {
			action_chat: Some(a),
			..Default::default()
		}));
	}
}
