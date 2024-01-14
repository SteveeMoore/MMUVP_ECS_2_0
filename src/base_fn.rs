#![allow(dead_code)]
#![allow(unused_macros)]

use std::{fs, time::Duration};

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

pub fn time_remaining(current_step: u64, current_time: Duration, last_step: u64) -> String {
    if current_step >= last_step {
        // Если текущий шаг больше или равен последнему, цикл завершился.
        return "0 минут 0 секунд".to_string()
    }

    let total_steps = last_step - current_step;
    
    if total_steps == 0 {
        // Избегаем деления на ноль, если всего один шаг.
        return "0 минут 0 секунд".to_string()
    }

    let estimated_duration_per_step = current_time.as_secs() / total_steps;

    // Оценка оставшегося времени на основе времени, прошедшего с начала цикла.
    let remaining_steps = last_step - current_step - 1;
    let remaining_time = estimated_duration_per_step * remaining_steps;

    let minutes = remaining_time / 60;
    let seconds = remaining_time % 60;

    format!("{} минут {} секунд", minutes, seconds)
}

pub fn print_current_sys(current_time: Duration, current_step: u64, last_step:u64, din: String, sigma: String){

    let curr_mins = current_time.as_secs() / 60;
    let curr_secs = current_time.as_secs() % 60;
    let string_time = format!("{} мин {} сек", curr_mins, curr_secs);
    let string_step = format!("{}/{}", current_step, last_step);

    let string_time_remaining:String = time_remaining(current_step, current_time, last_step);

    print!(
"****************************************************************************************\n
 Текущее время: {}. Текущий шаг: {}. Осталось времени: {}\n
 Напряжения:\n{} Деформации:\n{}\n
****************************************************************************************\n", string_time, string_step, string_time_remaining, sigma, din);
}