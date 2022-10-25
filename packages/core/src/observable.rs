use serde::Serialize;
use std::fmt;
use wasm_bindgen::prelude::*;

pub struct Observable<T> {
    pub payload: T,
    watch_list: Vec<js_sys::Function>,
}

impl<T> fmt::Debug for Observable<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "payload: {:?}\nwatch_list: {:?}",
            self.payload, self.watch_list
        )
    }
}

impl<T> Observable<T>
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
