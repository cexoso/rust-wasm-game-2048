use rand::prelude::*;
use std::fmt;
/**
 * this help provider random ability
 */
pub struct RandUtil {
    cheating_value_list: Vec<u32>,
    cheating_position_list: Vec<usize>,
    rng: ThreadRng,
}

impl fmt::Debug for RandUtil {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cheating_list: {:?}", self.cheating_value_list)
    }
}

impl RandUtil {
    pub fn new() -> Self {
        Self {
            cheating_value_list: Vec::new(),
            cheating_position_list: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    #[allow(dead_code)]
    pub fn set_next_value(&mut self, ns: Vec<u32>) {
        ns.iter().for_each(|n| {
            self.cheating_value_list.push(*n);
        })
    }

    pub fn get_rand_value(&mut self) -> u32 {
        let len = self.cheating_value_list.len();
        if len != 0 {
            return self.cheating_value_list.remove(0);
        }
        let result = [2, 4, 8, 16];
        let y: usize = self.rng.gen_range(0..4);
        result[y]
    }
}

impl RandUtil {
    #[allow(dead_code)]
    pub fn set_next_position(&mut self, ns: Vec<usize>) {
        ns.iter().for_each(|n| {
            self.cheating_position_list.push(*n);
        })
    }

    pub fn get_rand_position(&mut self, max_value: Option<usize>) -> usize {
        let len = self.cheating_position_list.len();
        if len != 0 {
            return self.cheating_position_list.remove(0);
        }
        let max_value = match max_value {
            Some(n) => n,
            None => 16,
        };
        self.rng.gen_range(0..=max_value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn cheating_list() {
        let mut rand = RandUtil::new();
        rand.set_next_value(vec![1, 2, 3]);
        assert_eq!(rand.get_rand_value(), 1);
        assert_eq!(rand.get_rand_value(), 2);
        assert_eq!(rand.get_rand_value(), 3);
    }

    #[test]
    fn cheating_position() {
        let mut rand = RandUtil::new();
        rand.set_next_position(vec![1, 1, 3]); // 数值表示下一个空的位置
        assert_eq!(rand.get_rand_position(Some(1)), 1);
        assert_eq!(rand.get_rand_position(Some(1)), 1);
        assert_eq!(rand.get_rand_position(Some(1)), 3);
    }
}
