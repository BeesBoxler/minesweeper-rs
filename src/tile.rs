use std::fmt::Error;

#[derive(Debug, Default, PartialEq)]
pub enum TileType {
    #[default]
    Undefined,
    Bomb,
    Empty(u8),
}

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Revealed,
    Flagged,
    Question,
    // #[default]
    Hidden,
}

#[derive(Default)]
pub struct Tile {
    state: State,
    tile_type: TileType,
}

impl Tile {
    pub fn make_bomb(&mut self) {
        self.tile_type = TileType::Bomb;
    }

    pub fn is_bomb(&self) -> bool {
        self.tile_type == TileType::Bomb
    }

    pub fn set_count(&mut self, count: u8) {
        self.tile_type = TileType::Empty(count)
    }

    pub fn reveal(&mut self) {
        self.state = State::Revealed
    }

    pub fn flag(&mut self) {
        self.state = State::Flagged
    }

    pub fn question(&mut self) {
        self.state = State::Question
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), Error> {
        let c = match self.state {
            State::Hidden => '\u{2588}',
            State::Flagged => '☂',
            State::Question => '?',
            State::Revealed => match self.tile_type {
                TileType::Undefined => ' ',
                TileType::Bomb => '✖',
                TileType::Empty(i) => char::from_u32(i as u32 + 48).unwrap(),
            },
        };

        write!(formatter, "{c}")
    }
}
