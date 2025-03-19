use super::field::Field;

pub struct Game<F> {
    width: usize,
    height: usize,
    bombs: usize,
    field: F,
}

#[derive(Debug, Default)]
pub struct Uninitialized;
pub struct Initialized(Field);

type NewGame = Game<Uninitialized>;

impl Game<Initialized> {
    pub fn get_field(&self) -> &Field {
        &self.field.0
    }
}

impl NewGame {
    pub fn easy() -> NewGame {
        Game {
            width: 8,
            height: 8,
            bombs: 10,
            field: Uninitialized,
        }
    }

    pub fn medium() -> NewGame {
        Game {
            width: 16,
            height: 16,
            bombs: 40,
            field: Uninitialized,
        }
    }

    pub fn init(self) -> Game<Initialized> {
        let field = Field::new(self.width, self.height, self.bombs);

        Game {
            width: self.width,
            height: self.height,
            bombs: self.bombs,
            field: Initialized(field),
        }
    }
}
