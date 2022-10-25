use crate::observable::Observable;
use crate::rand::RandUtil;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    checkerboard: Observable<[[u32; 4]; 4]>,
    rand: RandUtil,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            checkerboard: Observable::new([[0; 4]; 4]),
            rand: RandUtil::new(),
        }
    }
    pub fn init(&mut self) {
        self.checkerboard.payload = [[0; 4]; 4];
        let count = self.rand.get_rand_value(Some(2));
        for _ in 0..count {
            self.generate_one_cube();
        }
        self.checkerboard.notify_all();
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
        let remain_size = Self::get_empty_position(self.checkerboard.payload);
        if remain_size > 0 {
            let (x, j) = Self::get_position_by_offset(
                self.checkerboard.payload,
                self.rand.get_rand_position(Some(remain_size)),
            );
            let value = self.rand.get_rand_value(None);
            self.checkerboard.payload[x][j] = value;
            return true;
        }
        false
    }

    pub fn get_checkerboard_js_state(&self) -> JsValue {
        self.checkerboard.get_js_state()
    }

    pub fn get_checkerboard_state(&self) -> String {
        self.checkerboard.get_state()
    }

    pub fn subscript_board(&mut self, f: js_sys::Function) -> usize {
        self.checkerboard.subscript(f)
    }

    pub fn unsubscript_board(&mut self, f: js_sys::Function) -> bool {
        self.checkerboard.unsubscript(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn init() {
        let mut game = Game::new();
        assert_eq!(
            game.get_checkerboard_state(),
            "[[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
        // 第一个值表示初始化时随机生成多少次
        game.rand.set_next_value(vec![2, 2, 4]);
        game.rand.set_next_position(vec![1, 1]);
        game.init();
        assert_eq!(
            game.get_checkerboard_state(),
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
            game.get_checkerboard_state(),
            "[[1,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
    }
    #[test]
    fn generate_one_full() {
        let mut game = Game::new();
        game.rand.set_next_value(vec![1]);
        game.rand.set_next_position(vec![1]);
        game.checkerboard.payload = [[1; 4]; 4];
        // 满了，不允许再新增
        assert_eq!(game.generate_one_cube(), false);
    }
}
