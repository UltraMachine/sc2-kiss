use sc2_prost::{Action, ActionChat, action_chat::Channel};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ChatMessage {
	pub message: String,
	pub private: bool,
}
impl From<ChatMessage> for ActionChat {
	fn from(m: ChatMessage) -> Self {
		Self {
			channel: if m.private { Channel::Team as i32 } else { 0 },
			message: m.message,
		}
	}
}
impl From<ChatMessage> for Action {
	fn from(m: ChatMessage) -> Self {
		Self {
			action_chat: Some(m.into()),
			..Default::default()
		}
	}
}

#[macro_export]
macro_rules! chat {
	($($arg:tt)*) => {
		$crate::action::chat::ChatMessage {
			message: format!($($arg)*),
			private: false
		}
	};
}
#[macro_export]
macro_rules! chat_team {
	($($arg:tt)*) => {
		$crate::action::chat::ChatMessage {
			message: format!($($arg)*),
			private: true
		}
	};
}
pub use {chat, chat_team};
