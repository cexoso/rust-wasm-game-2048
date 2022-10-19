use wasm_bindgen::prelude::*;

use serde::Serialize;

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
    pub fn add_element(&self) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let div_element = document.create_element("div").unwrap();
        div_element.set_inner_html(&format!("hello world, {}", self.sum)[..]);
        let body = document.body().unwrap();
        body.append_child(&web_sys::Node::from(div_element))
            .unwrap();
        info!("log from rust {}", self.sum);
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

