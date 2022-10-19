use rand::Rng;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct GameMatrix {
    arr: Vec<Vec<u32>>,
    count: u32,
    empty_pos: Vec<String>,
}

// 对数据的基本操作
impl GameMatrix {
    pub fn new(row: usize, col: usize) -> GameMatrix {
        let col_vec = vec![0; col]; 
        let row_vec = vec![col_vec; row];
        
        GameMatrix {
            arr: row_vec,
            count: 0,
            empty_pos: vec![],
        }
    }

    pub fn init_nums(&mut self, count: u32) {
        for _i in 0..count {
            let (row, col) = self.rng_row_col_by_len();
            let value = self.rng_value();

            if let Some(v) = self.get_value(row, col) {
                if *v == 0 {
                    self.arr[row][col] = value;
                    self.count += 1; 
                }
            }
        }

        self.gen_empty_pos();
    }

    pub fn get_value(&self, row: usize, col: usize) -> Option<&u32> {
        if let Some(col_vec) = self.arr.get(row) {
            col_vec.get(col)
        } else {
            None
        }
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn get_empty_pos(&self) -> &[String] {
        &self.empty_pos
    }

    fn gen_empty_pos(&mut self) {
       self.arr.iter().enumerate().for_each(|(row, col_vec)| {
           col_vec.iter().enumerate().for_each(|(col, value)| {
               if *value == 0 {
                   self.empty_pos.push(format!("{}-{}", row, col));
               }
           })
       })
    }

}

// 随机能力
impl GameMatrix {
    fn rng_row_col_by_len(&self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let row: usize = rng.gen_range(0..self.arr.len());
        let col: usize = rng.gen_range(0..self.arr[0].len());

        (row, col)
    }

    fn rng_value(&self) -> u32 {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..10) {
            7..=10 => 4_u32,
            _ => 2_u32,
        }
    }

    fn rng_set_empty_value(&mut self, count: usize) {
        let max = if count > self.empty_pos.len() { self.empty_pos.len() } else { count };
        for _i in 0..max {
            let mut rng = rand::thread_rng();
            let i: usize = rng.gen_range(0..max);
            let arr: Vec<&str> = self.empty_pos[i].split('-').collect();
            let row = arr[0].parse::<usize>().unwrap();
            let col = arr[1].parse::<usize>().unwrap();

            self.arr[row][col] = self.rng_value();
            self.count += 1;
            self.empty_pos.remove(i);
        }

    }

}

// 数据转换能力
impl GameMatrix {
    pub fn transform(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.transform_left(),
            Direction::Top => self.transform_top(),
            Direction::Right => self.transform_right(),
            Direction::Bottom => self.transform_bottom(),
        } 
    }

    fn transform_left(&mut self) {

    }


    fn transform_right(&mut self) {

    }


    fn transform_top(&mut self) {

    }

    fn transform_bottom(&mut self) {

    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let game_matrix = GameMatrix::new(5, 5);
        assert_eq!(game_matrix.arr.len(), 5);
        assert_eq!(game_matrix.arr[0].len(), 5);
        assert_eq!(*game_matrix.get_value(0, 0).unwrap(), 0);
    }

    #[test]
    fn test_init() {
        let mut game_matrix = GameMatrix::new(5, 5);
        game_matrix.init_nums(2);

        assert_ne!(game_matrix.get_count(), 0);
    }

    #[test]
    fn test_random_gen() {
        let mut game_matrix = GameMatrix::new(5, 5);
        game_matrix.init_nums(2);

        assert_eq!(game_matrix.get_empty_pos().len(), 23);

        game_matrix.rng_set_empty_value(2);

        assert_ne!(game_matrix.get_empty_pos().len(), 23);

        println!("{:?}", game_matrix.arr);
        println!("{:?}", game_matrix.get_empty_pos());
    }
}

