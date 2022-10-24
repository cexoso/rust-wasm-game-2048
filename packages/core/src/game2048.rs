use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Debug)]
pub struct Game {
    checkerboard: [[u64; 4]; 4],
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Self {
            checkerboard: [[0; 4]; 4],
        }
    }
    pub fn init(&mut self) {
        // TODO: 先不实现随机函数 随机一个棋盘
        self.checkerboard[1][2] = 4;
    }
    pub fn get_checkerboard(&self) -> String {
        match serde_json::to_string(&self.checkerboard) {
            Ok(str) => str,
            Err(_) => String::from(""),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn init() {
        println!("hello world {}", 123);
        let mut game = Game::new();
        assert_eq!(
            game.get_checkerboard(),
            "[[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
        game.init();
        assert_eq!(
            game.get_checkerboard(),
            "[[0,0,0,0],[0,0,4,0],[0,0,0,0],[0,0,0,0]]"
        );
    }
}
