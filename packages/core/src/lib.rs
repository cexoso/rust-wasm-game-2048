#[macro_use]
extern crate log;
extern crate wasm_bindgen;
extern crate rand;

use wasm_bindgen::prelude::*;

mod mine_clean;
mod demo;

mod game;

use game::main as game_main;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn add(a: u8, b: u8) -> u8;
}

pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen(start)]
pub fn wasm_main() {
    // 初始化日志模块

    // 只是为了不报警告
    game_main();

    console_log::init_with_level(log::Level::Trace).unwrap();

    // 输出一句日志（显示在浏览器 console 中）

    debug!("Initialized!");
}
