use crate::rand::RandUtil;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Debug)]
pub struct Game {
    checkerboard: [[u32; 4]; 4],
    #[serde(skip)]
    rand: RandUtil,
    #[serde(skip)]
    watch_list: Vec<js_sys::Function>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            checkerboard: [[0; 4]; 4],
            rand: RandUtil::new(),
            watch_list: Vec::new(),
        }
    }
    pub fn init(&mut self) {
        self.checkerboard = [[0; 4]; 4];
        let count = self.rand.get_rand_value(Some(2));
        for _ in 0..count {
            self.generate_one_cube();
        }
        self.notify_all();
    }

    // TODO: 去掉这个用于测试的函数
    pub fn get_checkerboard(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.checkerboard).unwrap()
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
            let value = self.rand.get_rand_value(None);
            self.checkerboard[x][j] = value;
            return true;
        }
        false
    }

    pub fn subscript(&mut self, f: js_sys::Function) -> usize {
        self.watch_list.push(f);
        self.watch_list.len()
    }

    pub fn notify_all(&self) {
        let len = self.watch_list.len();
        if len == 0 {
            return;
        }
        let x = serde_wasm_bindgen::to_value(&self.checkerboard).unwrap();
        for index in 0..len {
            let f = &self.watch_list[index];
            let this = JsValue::NULL;
            f.call1(&this, &x).unwrap();
        }
    }

    pub fn unsubscript(&mut self, f: js_sys::Function) -> bool {
        let watch_list = &self.watch_list;
        for i in 0..watch_list.len() {
            let spec_f = &watch_list[i];
            if *spec_f == f {
                self.watch_list.remove(i);
                return true;
            }
        }
        return false;
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
