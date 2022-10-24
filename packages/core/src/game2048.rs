use crate::rand::RandUtil;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Debug)]
pub struct Game {
    checkerboard: [[u32; 4]; 4],
    #[serde(skip)]
    rand: RandUtil,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Self {
            checkerboard: [[0; 4]; 4],
            rand: RandUtil::new(),
        }
    }
    pub fn init(&mut self) {
        self.checkerboard = [[0; 4]; 4];
        let count = self.rand.get_rand_value();
        for _ in 0..count {
            self.generate_one_cube();
        }
    }

    pub fn get_checkerboard(&self) -> String {
        match serde_json::to_string(&self.checkerboard) {
            Ok(str) => str,
            Err(_) => String::from(""),
        }
    }

    fn get_empty_position(checkerboard: [[u32; 4]; 4]) -> usize {
        let mut size = 0;
        for i in 0..checkerboard.len() {
            let row = checkerboard[i];
            for j in 0..row.len() {
                if row[j] == 0 {
                    size += 1
                }
            }
        }
        size
    }

    fn get_position_by_offset(checkerboard: [[u32; 4]; 4], mut offset: usize) -> (usize, usize) {
        for i in 0..checkerboard.len() {
            let row = checkerboard[i];
            for j in 0..row.len() {
                if row[j] == 0 {
                    offset -= 1;
                }
                if offset == 0 {
                    return (i, j);
                }
            }
        }
        (0, 0)
    }

    pub fn generate_one_cube(&mut self) -> bool {
        let remain_size = Self::get_empty_position(self.checkerboard);
        if remain_size > 0 {
            let (x, j) = Self::get_position_by_offset(
                self.checkerboard,
                self.rand.get_rand_position(Some(remain_size)),
            );
            let value = self.rand.get_rand_value();
            self.checkerboard[x][j] = value;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn init() {
        let mut game = Game::new();
        assert_eq!(
            game.get_checkerboard(),
            "[[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
        // 第一个值表示初始化时随机生成多少次
        game.rand.set_next_value(vec![2, 2, 4]);
        game.rand.set_next_position(vec![1, 1]);
        game.init();
        assert_eq!(
            game.get_checkerboard(),
            "[[2,4,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
    }
    #[test]
    fn generate_one_cube() {
        let mut game = Game::new();
        game.rand.set_next_value(vec![1]);
        game.rand.set_next_position(vec![1]);
        game.generate_one_cube();
        assert_eq!(
            game.get_checkerboard(),
            "[[1,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
    }
    #[test]
    fn generate_one_full() {
        let mut game = Game::new();
        game.rand.set_next_value(vec![1]);
        game.rand.set_next_position(vec![1]);
        game.checkerboard = [[1; 4]; 4];
        // 满了，不允许再新增
        assert_eq!(game.generate_one_cube(), false);
    }
}
