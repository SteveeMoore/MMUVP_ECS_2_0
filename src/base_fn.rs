#![allow(dead_code)]
#![allow(unused_macros)]

use std::{fs, time::Duration};

use crate::mmuvp::elasticity::components::{SigmaComponent, EpsComponent};

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
  
    let estimated_duration_per_step = current_time.as_secs_f64() / (current_step as f64);
    // Оценка оставшегося времени на основе времени, прошедшего с начала цикла.
    let remaining_steps = (last_step - current_step - 1) as f64;
    let remaining_time = (estimated_duration_per_step * remaining_steps) as u64;

    let minutes = remaining_time / 60;
    let seconds = remaining_time % 60;

    format!("{} минут {} секунд", minutes, seconds)
}

pub fn print_current_sys(current_time: Duration, step: i64, last_step:i64, polycrystal_eps: &EpsComponent, polycrystal_sigma: &SigmaComponent){
    let mut sigma = String::new();
    for (i, &x) in polycrystal_sigma.get_tensor().iter().enumerate() {
        let formatted = format!("{:.2}", x);
        sigma.push_str(&formatted);
        if i % 3 == 2 {
            sigma.push('\n');
        } else {
            sigma.push(' ');
        }
    }

    let mut eps = String::new();
    for (i, &x) in polycrystal_eps.get_tensor().iter().enumerate() {
        let formatted = format!("{:.2}", x);
        eps.push_str(&formatted);
        if i % 3 == 2 {
            eps.push('\n');
        } else {
            eps.push(' ');
        }
    }

    let current_step = step as u64;
    let last_step = last_step as u64;
    let curr_mins = current_time.as_secs() / 60;
    let curr_secs = current_time.as_secs() % 60;
    let string_time = format!("{} мин {} сек", curr_mins, curr_secs);
    let string_step = format!("{}/{}", current_step, last_step);

    let string_time_remaining:String = time_remaining(current_step, current_time, last_step);

    print!(
"****************************************************************************************\n
 Текущее время: {}. Текущий шаг: {}. Осталось времени: {}\n
 Напряжения:\n{} Деформации:\n{}\n
****************************************************************************************\n", string_time, string_step, string_time_remaining, sigma, eps);
}