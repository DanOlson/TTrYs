use super::{
    stateful_list::StatefulList,
    GameMode,
    Config
};

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Section {
    #[default]
    ChooseGameMode,
    ChooseInitialLevel,
    StartGame
}

pub struct ConfigBuilder<'a> {
    pub is_configured: bool,
    pub current_section: Section,
    pub sections: Vec<Section>,
    pub game_mode: GameMode,
    pub level_list: StatefulList<&'a str>
}

impl<'a> ConfigBuilder<'a> {
    pub fn build(&self) -> Config {
        Config {
            game_mode: self.game_mode,
            initial_level: self.level_list.state.selected().unwrap(),
        }
    }

    pub fn is_configured(&self) -> bool {
        self.is_configured
    }

    pub fn previous_section(&mut self) {
        match self.current_section {
            Section::StartGame => self.current_section = Section::ChooseInitialLevel,
            Section::ChooseInitialLevel => self.current_section = Section::ChooseGameMode,
            _ => {}
        }
    }

    pub fn next_section(&mut self) {
        match self.current_section {
            Section::ChooseGameMode => self.current_section = Section::ChooseInitialLevel,
            Section::ChooseInitialLevel => self.current_section = Section::StartGame,
            _ => {}
        }
    }

    pub fn on_left(&mut self) {
        match self.current_section {
            Section::ChooseGameMode => self.game_mode = GameMode::AType,
            Section::ChooseInitialLevel => self.level_list.previous(),
            _ => {}
        }
    }

    pub fn on_right(&mut self) {
        match self.current_section {
            Section::ChooseGameMode => self.game_mode = GameMode::BType,
            Section::ChooseInitialLevel => self.level_list.next(),
            _ => {}
        }
    }

    pub fn configured(&mut self) {
        if self.current_section == Section::StartGame {
            self.is_configured = true;
        }
    }
}

impl<'a> Default for ConfigBuilder<'a> {
    fn default() -> Self {
        let sections = vec![Section::default()];
        ConfigBuilder {
            is_configured: false,
            current_section: sections[0],
            sections,
            game_mode: GameMode::default(),
            level_list: StatefulList::with_items(vec![
                "Level 1",
                "Level 2",
                "Level 3",
                "Level 4",
                "Level 5",
                "Level 6",
                "Level 7",
                "Level 8",
                "Level 9",
                "Level 10",
            ])
        }
    }
}
