mod matrix;

use matrix::*;

pub fn main() {
    let mut game_matrix = GameMatrix::new(5, 5);
    game_matrix.init_nums(5);

    game_matrix.transform(Direction::Top);
    game_matrix.transform(Direction::Left);
    game_matrix.transform(Direction::Right);
    game_matrix.transform(Direction::Bottom);
    game_matrix.get_count();
}
