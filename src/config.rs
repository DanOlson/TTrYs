mod stateful_list;
mod builder;

pub use builder::{ConfigBuilder, Section};

#[derive(Clone, Copy, Default, PartialEq)]
pub enum GameMode {
    #[default]
    AType,
    BType,
}

pub struct Config {
    pub game_mode: GameMode,
    pub initial_level: usize,
}

impl Config {
    pub fn builder() -> ConfigBuilder<'static> {
        ConfigBuilder::default()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            game_mode: GameMode::AType,
            initial_level: 0
        }
    }
}
