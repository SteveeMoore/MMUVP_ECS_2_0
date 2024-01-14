#![allow(dead_code)]
extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;

pub struct Params {
    params: HashMap<String, Value>,
}

impl Params {
    pub fn new() -> Self {
        Params {
            params: HashMap::new(),
        }
    }

    pub fn set_hash_map(&mut self, data: HashMap<String, Value>) {
        self.params = data;
    }

    pub fn get_f64(&self, key: &str) -> f64 {
        if let Some(value) = self.params.get(key) {
            if let Some(number) = value.as_f64() {
                return number;
            } else {
                println!("Значение ключа {} не является f64", key)
            }
        } else {
            println!("Ключ {} не найден", key)
        }
        0.0
    }
    pub fn get_i64(&self, key: &str) -> i64 {
        if let Some(value) = self.params.get(key) {
            if let Some(number) = value.as_i64() {
                return number;
            } else {
                println!("Значение ключа {} не является i64", key)
            }
        } else {
            println!("Ключ {} не найден", key)
        }
        0
    }
}