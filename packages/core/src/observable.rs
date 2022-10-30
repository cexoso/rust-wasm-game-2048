use std::fmt;
use wasm_bindgen::prelude::*;

type Matrix = [[u32; 4]; 4];

pub struct Observable {
    pub payload: Matrix,
    watch_list: Vec<js_sys::Function>,
}

impl fmt::Debug for Observable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.payload;

        let print_str = x.iter().map(|col_arr| {
            col_arr.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")
        }).collect::<Vec<String>>().join("\n");

        write!( f, "{}", print_str)
    }
}

impl Observable {
    pub fn new(payload: Matrix) -> Self {
        Self {
            payload,
            watch_list: Vec::new(),
        }
    }
    pub fn update<F>(&mut self, updater: F)
    where
        F: Fn(&mut Matrix),
    {
        updater(&mut self.payload);
        self.notify_all();
    }
    pub fn subscript(&mut self, f: js_sys::Function) -> usize {
        self.watch_list.push(f);
        self.watch_list.len()
    }

    pub fn get_js_state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.payload).unwrap()
    }

    pub fn notify_all(&self) {
        let len = self.watch_list.len();
        if len == 0 {
            return;
        }
        let state = self.get_js_state();
        for index in 0..len {
            let f = &self.watch_list[index];
            let this = JsValue::NULL;
            f.call1(&this, &state).unwrap();
        }
    }

    pub fn unsubscript(&mut self, f: js_sys::Function) -> bool {
        let watch_list = &self.watch_list;
        for (i, v) in watch_list.iter().enumerate() {
            if *v == f {
                self.watch_list.remove(i);
                return true;
            }
        }
        false
    }
}
