use serde::Serialize;
use std::fmt;
use wasm_bindgen::prelude::*;

type T = [[u32; 4]; 4];

pub struct Observable {
    pub payload: T,
    watch_list: Vec<js_sys::Function>,
}

impl fmt::Debug for Observable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.payload;
        write!(
            f,
            "\n{},{},{},{}\n{},{},{},{}\n{},{},{},{}\n{},{},{},{}",
            x[0][0], x[0][1], x[0][2], x[0][3],
            x[1][0], x[1][1], x[1][2], x[1][3],
            x[2][0], x[0][1], x[2][2], x[2][3],
            x[3][0], x[3][1], x[3][2], x[3][3],
        )
    }
}

impl Observable
where
    T: Serialize,
{
    pub fn new(payload: T) -> Self {
        Self {
            payload,
            watch_list: Vec::new(),
        }
    }
    pub fn update<F>(&mut self, updater: F)
    where
        F: Fn(&mut T) -> (),
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

    pub fn get_state(&self) -> String {
        match serde_json::to_string(&self.payload) {
            Ok(str) => str,
            Err(_) => String::from(""),
        }
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
