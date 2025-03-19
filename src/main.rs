mod field;
mod game;
mod tile;

fn main() {
    let game = game::Game::easy().init();
    print!("{}", game.get_field());

    let game = game::Game::medium().init();
    print!("{}", game.get_field())
}
