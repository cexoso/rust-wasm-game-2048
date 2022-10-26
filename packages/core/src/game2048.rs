use super::matrix::Matrix;
use crate::observable::Observable;
use crate::rand::RandUtil;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    checkerboard: Observable,
    rand: RandUtil,
    random_when_move: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            checkerboard: Observable::new([[0; 4]; 4]),
            rand: RandUtil::new(),
            random_when_move: false,
        }
    }

    pub fn set_random_when_move(&mut self, next: bool) {
        self.random_when_move = next;
    }
    pub fn up(&mut self) {
        self.pure_up();
        self.generate_cube(2);
        self.checkerboard.notify_all();
    }
    pub fn down(&mut self) {
        self.pure_down();
        self.generate_cube(2);
        self.checkerboard.notify_all();
    }
    pub fn left(&mut self) {
        self.pure_left();
        self.generate_cube(2);
        self.checkerboard.notify_all();
    }
    pub fn right(&mut self) {
        self.pure_right();
        self.generate_cube(2);
        self.checkerboard.notify_all();
    }
    pub fn pure_up(&mut self) {
        Matrix::rotate_right(&mut self.checkerboard.payload);
        self.pure_right();
        Matrix::rotate_left(&mut self.checkerboard.payload);
    }

    pub fn pure_down(&mut self) {
        Matrix::rotate_left(&mut self.checkerboard.payload);
        self.pure_right();
        Matrix::rotate_right(&mut self.checkerboard.payload);
    }

    pub fn generate_cube(&mut self, max: u32) -> bool {
        let mut result = false;
        for _ in 0..max {
            let success = self.generate_one_cube();
            if success {
                result = success;
            } else {
                return result;
            }
        }
        result
    }

    pub fn pure_left(&mut self) {
        Matrix::horizontal_reverse(&mut self.checkerboard.payload);
        self.pure_right();
        Matrix::horizontal_reverse(&mut self.checkerboard.payload);
    }
    pub fn pure_right(&mut self) {
        let checkerboard = &mut self.checkerboard.payload;
        for row_index in 0..checkerboard.len() {
            let row_len = checkerboard[row_index].len();
            let mut floor_value: i32 = -1;
            let mut floor_zero: i32 = -1;
            for col in 0..row_len {
                let col_index = row_len - col - 1;
                let col_index_i32 = col_index as i32;
                let value = checkerboard[row_index][col_index];
                // 当前遍历的是 0
                if value == 0 {
                    // 更新 0 位置
                    if floor_zero == -1 {
                        floor_zero = col_index_i32;
                    }
                    continue;
                }

                // 非 0 情况下, 若不存在上一个值
                // 更新上一个值的位置
                if floor_value == -1 {
                    // 尝试移动到上一个 0 值位置，并更新数据
                    if floor_zero != -1 {
                        checkerboard[row_index][floor_zero as usize] = value;
                        checkerboard[row_index][col_index] = 0;
                        // 移动到上一个 0 值位置
                        // 0 值位置往前移一位(下一位必是 0 值)
                        floor_value = floor_zero;
                        floor_zero = floor_value - 1;
                        continue;
                    }
                    floor_value = col_index_i32;
                    continue;
                }
                // 否则尝试合并和移动
                let floor_value_usize = floor_value as usize;
                let x = checkerboard[row_index][floor_value_usize];
                // 值相同,合并
                if x == value {
                    checkerboard[row_index][floor_value_usize] = 2 * value;
                    checkerboard[row_index][col_index] = 0; // 删除原值
                    if floor_zero == -1 {
                        floor_zero = col_index_i32;
                    }
                    floor_value = -1;
                    continue;
                }
                // 值不同,尝试移动到上一个 0 值位置
                if floor_zero != -1 {
                    checkerboard[row_index][floor_zero as usize] = value;
                    checkerboard[row_index][col_index] = 0;
                    floor_zero = -1;
                }
                // 找不到 0 值位置可以移动
                // 这种情况下说明是一个值接着一个值
                // 所以更新有值块的位置信息
                floor_value = col_index_i32;
            }
        }
    }

    pub fn init(&mut self) {
        self.set_random_when_move(true);
        self.checkerboard.update(|checkerboard| {
            *checkerboard = [[0; 4]; 4];
        });
        let count = self.rand.get_rand_value(Some(2));
        for _ in 0..count {
            self.generate_one_cube();
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
        let origin_offset = offset;
        for i in 0..checkerboard.len() {
            let row = checkerboard[i];
            for j in 0..row.len() {
                if row[j] == 0 {
                    if offset == 0 {
                        return (i, j);
                    }
                    offset -= 1;
                }
            }
        }
        println!("checkerboard: {:?}", checkerboard);
        panic!("can't get position with offset: {}", origin_offset)
    }

    pub fn generate_one_cube(&mut self) -> bool {
        let remain_size = Self::get_empty_position(self.checkerboard.payload);
        if remain_size > 0 {
            let (x, j) = Self::get_position_by_offset(
                self.checkerboard.payload,
                self.rand.get_rand_position(Some(remain_size - 1)),
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
        game.rand.set_next_position(vec![0, 0]);
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
        game.rand.set_next_position(vec![0]);
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
        game.rand.set_next_position(vec![0]);
        game.checkerboard.payload = [[1; 4]; 4];
        // 满了，不允许再新增
        assert_eq!(game.generate_one_cube(), false);
    }
    #[test]
    fn right_1() {
        let mut game = Game::new();
        game.checkerboard.payload = [[0, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        game.pure_right();
        assert_eq!(
            game.get_checkerboard_state(),
            "[[0,0,1,2],[0,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
    }

    #[test]
    fn right_2() {
        let mut game = Game::new();
        game.checkerboard.payload = [[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        game.pure_right();
        assert_eq!(
            game.get_checkerboard_state(),
            "[[0,0,2,2],[0,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
    }

    #[test]
    fn right_3() {
        let mut game = Game::new();
        game.checkerboard.payload = [[1, 0, 0, 2], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        game.pure_right();
        assert_eq!(
            game.get_checkerboard_state(),
            "[[0,0,1,2],[0,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
    }

    #[test]
    fn right_4() {
        let mut game = Game::new();
        game.checkerboard.payload = [[0, 0, 0, 0], [4, 2, 8, 8], [0, 0, 0, 0], [0, 0, 0, 0]];
        game.pure_right();
        assert_eq!(
            game.get_checkerboard_state(),
            "[[0,0,0,0],[0,4,2,16],[0,0,0,0],[0,0,0,0]]"
        );
    }

    #[test]
    fn up() {
        let mut game = Game::new();
        game.checkerboard.payload = [[1, 0, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        game.pure_up();
        assert_eq!(
            game.get_checkerboard_state(),
            "[[2,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
    }

    #[test]
    fn left() {
        let mut game = Game::new();
        game.checkerboard.payload = [[0, 0, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        game.pure_left();
        assert_eq!(
            game.get_checkerboard_state(),
            "[[0,0,0,0],[2,0,0,0],[0,0,0,0],[0,0,0,0]]"
        );
    }
    #[test]
    fn down() {
        let mut game = Game::new();
        game.checkerboard.payload = [[0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        game.pure_down();
        assert_eq!(
            game.get_checkerboard_state(),
            "[[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,2,0,0]]"
        );
    }

    #[test]
    fn generate_cube() {
        let mut game = Game::new();
        game.rand.set_next_value(vec![1]);
        game.rand.set_next_position(vec![0]);
        game.checkerboard.payload = [[1; 4], [1; 4], [1; 4], [0, 1, 1, 1]];

        // 只要能生成一个元素就算是成功
        assert_eq!(game.generate_cube(2), true);
        assert_eq!(
            game.get_checkerboard_state(),
            "[[1,1,1,1],[1,1,1,1],[1,1,1,1],[1,1,1,1]]"
        );
    }
}
