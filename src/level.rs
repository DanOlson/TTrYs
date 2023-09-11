use crate::{
    scoring::ScoringConfig,
    piece::{Shape, Piece},
};

const NUM_LEVELS: usize = 10;
const MAX_TICKS_PER_DROP: usize = 61;
const THEMES: [Theme; 10] = [
    Theme {
        orange_ricky: 214,
        blue_ricky: 12,
        cleveland_z: 9,
        rhode_island_z: 2,
        hero: 14,
        smashboy: 11,
        teewee: 56
    },
    Theme {
        orange_ricky: 166,
        blue_ricky: 61,
        cleveland_z: 88,
        rhode_island_z: 64,
        hero: 39,
        smashboy: 227,
        teewee: 57
    },
    Theme {
        orange_ricky: 172,
        blue_ricky: 69,
        cleveland_z: 160,
        rhode_island_z: 76,
        hero: 45,
        smashboy: 229,
        teewee: 53
    },
    Theme {
        orange_ricky: 178,
        blue_ricky: 27,
        cleveland_z: 161,
        rhode_island_z: 78,
        hero: 41,
        smashboy: 227,
        teewee: 55
    },
    Theme {
        orange_ricky: 167,
        blue_ricky: 20,
        cleveland_z: 124,
        rhode_island_z: 71,
        hero: 51,
        smashboy: 220,
        teewee: 52
    },
    Theme {
        orange_ricky: 215,
        blue_ricky: 153,
        cleveland_z: 163,
        rhode_island_z: 34,
        hero: 44,
        smashboy: 184,
        teewee: 99
    },
    Theme {
        orange_ricky: 130,
        blue_ricky: 62,
        cleveland_z: 196,
        rhode_island_z: 10,
        hero: 50,
        smashboy: 190,
        teewee: 92
    },
    Theme {
        orange_ricky: 58,
        blue_ricky: 61,
        cleveland_z: 52,
        rhode_island_z: 65,
        hero: 75,
        smashboy: 101,
        teewee: 96
    },
    Theme {
        orange_ricky: 136,
        blue_ricky: 67,
        cleveland_z: 125,
        rhode_island_z: 119,
        hero: 123,
        smashboy: 185,
        teewee: 183
    },
    Theme {
        orange_ricky: 179,
        blue_ricky: 147,
        cleveland_z: 162,
        rhode_island_z: 144,
        hero: 195,
        smashboy: 230,
        teewee: 225
    },
];

#[derive(Copy, Clone)]
pub struct Level {
    pub ticks_per_drop: usize,
    pub counter: usize,
    pub scoring_config: ScoringConfig,
    pub rows_to_pass: usize,
    pub number: usize,
    pub theme: Theme,
}

impl Level {
    pub fn level_count() -> usize { NUM_LEVELS }

    pub fn all() -> Vec<Self> {
        (0..Self::level_count())
            .map(|i| Level::new(i + 1))
            .collect()
    }

    pub fn new(
        number: usize,
    ) -> Self {
        Self {
            number,
            ticks_per_drop: MAX_TICKS_PER_DROP - number * 4,
            counter: 0,
            rows_to_pass: number * 10,
            scoring_config: ScoringConfig::new(
                number * 40,
                number * 100,
                number * 300,
                number * 1200,
            ),
            theme: THEMES[number - 1]
        }
    }

    pub fn tick(&mut self) -> Option<()> {
        self.counter += 1;
        if self.counter < self.ticks_per_drop {
            None
        } else {
            self.counter = 0;
            Some(())
        }
    }
}

#[derive(Copy, Clone)]
pub struct Theme {
    pub orange_ricky: u8,
    pub blue_ricky: u8,
    pub cleveland_z: u8,
    pub rhode_island_z: u8,
    pub hero: u8,
    pub smashboy: u8,
    pub teewee: u8,
}

impl Theme {
    pub fn piece_color(&self, piece: &Piece) -> u8 {
        match piece.shape {
            Shape::OrangeRicky => self.orange_ricky,
            Shape::BlueRicky => self.blue_ricky,
            Shape::ClevelandZ => self.cleveland_z,
            Shape::RhodeIslandZ => self.rhode_island_z,
            Shape::Hero => self.hero,
            Shape::Smashboy => self.smashboy,
            Shape::Teewee => self.teewee,
        }
    }
}
