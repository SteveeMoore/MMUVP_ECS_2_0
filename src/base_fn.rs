#![allow(dead_code)]
#![allow(unused_macros)]

use std::fs;

use super::consts::FILE_OUTPUT_PATH;

#[macro_export]
macro_rules! create_component_map {
    ($map:ident, $entity_type:ty, $component_type:ty) => {
        let mut $map: HashMap<$entity_type, $component_type> = HashMap::new();
    };
}

#[macro_export]
macro_rules! insert_component {
    ($entity:expr, $component:expr, $map:ident) => {
        $map.insert($entity.clone(), $component);
    };
}

pub fn clear_output_folder(){
    // Получаем список файлов и директорий внутри указанной папки
    let entries = fs::read_dir(FILE_OUTPUT_PATH).expect("Ошибка открытия дирректории вывода");

    for entry in entries {
        let entry = entry.expect("Ошибка проверки наличия файла");
        let path = entry.path();

        // Проверяем, является ли элемент файлом
        if path.is_file() {
            // Удаляем файл
            fs::remove_file(&path).expect("Ошибка удаления файла");
        }
    }
}