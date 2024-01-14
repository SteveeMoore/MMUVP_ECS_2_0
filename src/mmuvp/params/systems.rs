#![allow(dead_code)]
use serde_json::{Value, from_str};
use std::{
    collections::HashMap, 
    fs, 
    path::PathBuf};

use crate::consts::FILE_INPUT_PATH;
use super::components::Params;


pub fn from_file(params: &mut Params) {
    // Прочитать JSON файл и считать его содержимое в виде строки
    let json_string = fs::read_to_string(PathBuf::from(FILE_INPUT_PATH).join("param.json")).expect("Ошибка открытия файла param.json");

    // Десериализовать JSON-строку в serde_json::Value
    let json_value: Value = from_str(&json_string).expect("Ошибка перевода строки json в значения");

    // Преобразовать Value в HashMap, если JSON представляет собой объект
    if let Some(map) = json_value.as_object() {
        let hashmap: HashMap<String, Value> = map.iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        params.set_hash_map(hashmap);
        
    } 
}