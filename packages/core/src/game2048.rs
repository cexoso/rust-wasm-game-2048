use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Self {}
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        Game::new();
    }
}
