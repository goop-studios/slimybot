use poise::serenity_prelude::{EmojiId, Message, ReactionType};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct Welcome {
    pub enabled: bool,
    pub channel: Option<u64>,
}

#[derive(Debug, Default)]
pub struct AutoRole {
    pub enabled: bool,
    pub role: Option<u64>,
}

#[derive(Debug, Default)]
pub struct ReactionRoles {
    pub reactions: Vec<ReactionParams>,
}

#[derive(Debug, Clone)]
pub struct ReactionParams {
    pub role: u64,
    pub emoji: ReactionType,
    pub message: Message,
}

#[derive(Debug)]
pub struct Data {
    pub config_dir: Arc<Mutex<String>>,
    // Section Welcome
    pub welcome: Arc<Mutex<Welcome>>,
    // Section AutoRole
    pub autorole: Arc<Mutex<AutoRole>>,
    // Section ReactionRoles
    pub reaction_roles: Arc<Mutex<ReactionRoles>>,
} // User data, which is stored and accessible in all command invocations

#[derive(Debug, Default, Clone)]
pub struct EmojiIdentifier2 {
    pub animated: bool,
    pub id: u64,
    pub name: String,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            config_dir: Arc::new(Mutex::new("config.toml".to_string())),
            welcome: Arc::new(Mutex::new(Welcome::default())),
            autorole: Arc::new(Mutex::new(AutoRole::default())),
            reaction_roles: Arc::new(Mutex::new(ReactionRoles::default())),
        }
    }
}

impl Default for ReactionParams {
    fn default() -> Self {
        ReactionParams {
            role: 0,
            emoji: ReactionType::Custom {
                animated: false,
                id: EmojiId::new(0),
                name: Some("".to_string()),
            },
            message: Message::default(),
        }
    }
}

impl ReactionRoles {
    pub fn add_reaction(&mut self, role: u64, emoji: ReactionType, message: Message) {
        self.reactions.push(ReactionParams {
            role,
            emoji,
            message,
        });
    }

    pub fn remove_reaction(&mut self, message: Message) {
        self.reactions
            .retain(|reaction| reaction.message.id != message.id);
    }

    pub fn new() -> Self {
        ReactionRoles {
            reactions: Vec::new(),
        }
    }
}
