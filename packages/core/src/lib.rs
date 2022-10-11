#[macro_use]
extern crate log;
extern crate wasm_bindgen;

use serde::Serialize;
use wasm_bindgen::prelude::*;

mod game2048;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn add(a: u8, b: u8) -> u8;
}

#[derive(Serialize)]
struct Point {
    x: i32,
    y: i32,
    z: Point1,
}

#[derive(Serialize)]
struct Point1 {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
pub struct Adder {
    sum: i32,
}

#[wasm_bindgen]
impl Adder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { sum: 0 }
    }
    pub fn add_callback(&self, f: &js_sys::Function) {
        let this = JsValue::NULL;
        f.call0(&this).unwrap();
    }
    pub fn return_point(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&Point {
            x: 1,
            y: 2,
            z: Point1 { x: 1, y: 2 },
        })
        .unwrap()
    }
    pub fn hello(&self) -> String {
        "hello".to_string()
    }
    pub fn add(&mut self, n: i32) -> i32 {
        self.sum += n;
        self.sum
    }
    pub fn big_sum(&self) -> i32 {
        let mut i = 0;
        info!("start");
        for num in 1..100000000 {
            if num % 23546845 == 0 {
                i += num;
            }
        }
        info!("end");
        i
    }
}

#[wasm_bindgen]
pub fn fib(num: u8) -> u64 {
    match num {
        1 => 1,
        2 => 1,
        _ => fib(num - 1) + fib(num - 2),
    }
}

pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen(start)]
pub fn wasm_main() {
    // 初始化日志模块

    console_log::init_with_level(log::Level::Trace).unwrap();

    // 输出一句日志（显示在浏览器 console 中）

    debug!("Initialized!");
}
