#![allow(dead_code)]

use std::{
    collections::HashMap, 
    fs::File, path::PathBuf, 
    io::{BufReader, BufRead}
};

use nalgebra::{Vector3, Matrix3};

use crate::{
    mmuvp::{entity::CrystalEntity, elasticity::components::SigmaComponent}, 
    consts::{FILE_INPUT_PATH, MEGA}};

use super::components::*;

pub fn initialize_burgers_vectors(
    burgers_map: &mut HashMap<CrystalEntity, BurgersVectorComponent>,
) {
    let file = File::open(PathBuf::from(FILE_INPUT_PATH).join("b.input")).expect("Ошибка открытия файла b.input");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Ошибка. Файл b.input заполнен неверно");
        let values: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse::<f64>().expect("Ошибка перевода строки b.input в числа"))
            .collect();
        if values.len()!=3 {
            panic!("Ошибка. Количество элементов в b.input равно {}",values.len());
        }
        let vector = Vector3::new(values[0], values[1], values[2]).normalize();    
        burgers_map.values_mut().for_each(|burgers_vector| {
            if index < 12 {
                burgers_vector.set_vector(index * 2, vector);
                burgers_vector.set_vector(index * 2 + 1, -vector);
            }
        });         
    }
}

pub fn initialize_normal_vectors(
    normals_map: &mut HashMap<CrystalEntity, NormalVectorComponent>,
) {
    let file = File::open(PathBuf::from(FILE_INPUT_PATH).join("n.input")).expect("Ошибка открытия файла n.input");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Ошибка. Файл n.input неверно заполнен");
        let values: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse::<f64>().expect("Ошибка перевода строки n.input в числа"))
            .collect();
        if values.len()!=3 {
            panic!("Ошибка. Количество элементов в n.input равно {}",values.len());
        }
        let vector = Vector3::new(values[0], values[1], values[2]).normalize();
        normals_map.values_mut().for_each(|normal_vector| {
            if index < 12 {
                normal_vector.set_vector(index * 2, vector);
                normal_vector.set_vector(index * 2 + 1, vector);
            }
        });
    }
}

pub fn initialize_bn(
    bn_map: &mut HashMap<CrystalEntity, BNComponent>,
    burgers_map: &HashMap<CrystalEntity, BurgersVectorComponent>,
    normals_map: &HashMap<CrystalEntity, NormalVectorComponent>,
){
    // Перебираем каждую сущность
    for (entity, bn_component) in bn_map.iter_mut() {
        if let Some(burgers_vector) = burgers_map.get(entity) {
            if let Some(normal_vector) = normals_map.get(entity) {
                // Вложенные циклы для перебора всех 24 индексов
                for index in 0..24 {
                    let b = burgers_vector
                        .get_vector(index)
                        .expect("Ошибка извлечения вектора Бюргерса");
                    let n = normal_vector
                        .get_vector(index)
                        .expect("Ошибка извлечения вектора Нормали");

                    let mut matrix = Matrix3::zeros(); // Создаем пустую матрицу
                    for (i, bi) in b.iter().enumerate() {
                        for (j, nj) in n.iter().enumerate() {
                            matrix[(i, j)] = bi * nj;
                        }
                    }
                    bn_component.set_matrix(index, matrix);
                }
            }
            else{
                panic!("Ошибка поиска компонента Нормали");
            }
        }
        else {
            panic!("Ошибка поиска компонента вектора Бюргерса");
        }
    }
}

pub fn calc_tau(
    tau_map: &mut HashMap<CrystalEntity, TauComponent>,
    bn_map: &HashMap<CrystalEntity, BNComponent>,
    sigma_map: &HashMap<CrystalEntity, SigmaComponent>,
) {
    for (entity, tau_component) in tau_map.iter_mut() {
        if let Some(bn_component) = bn_map.get(entity) {
            if let Some(sigma_component) = sigma_map.get(entity) {
                let sigma = sigma_component.get_tensor();
                for index in 0..24 {
                    let bn = bn_component
                        .get_matrix(index)
                        .expect("Ошибка получения матрицы bn");
                    let tau = bn.dot(&sigma);
                    if tau > 0.0 {
                        tau_component.set_values(index, tau);
                    } else {
                        tau_component.set_values(index, 0.0);
                    }
                }
            }
            else{
                panic!("Ошибка поиска компонента sigma");
            }
        }
        else{
            panic!("Ошибка поиска компонента bn");
        }
    }
}

pub fn initialize_tau_c(
    tau_c_map: &mut HashMap<CrystalEntity, TauComponent>,
    tauc: f64,
) {
    let value= tauc/MEGA;
    tau_c_map.values_mut().for_each(|tau_c| {
        for index in 0..24 {
            tau_c.set_values(index, value);
        }
    });
}

pub fn initialize_tau_c_hp(
    tau_c_map: &mut HashMap<CrystalEntity, TauComponent>,
    tauc: f64,
    b:f64,
    k_y: f64,
    d_g: f64,
){
    let value= tauc/MEGA;
    let addition_hp = k_y*(b / d_g).sqrt() / MEGA;
    tau_c_map.values_mut().for_each(|tau_c| {
        for index in 0..24 {
            tau_c.set_values(index, value + addition_hp);
        }
    });
}

pub fn calc_gamma(
    gamma_map: &mut HashMap<CrystalEntity,GammaComponent>,
    gamma_rate_map: &HashMap<CrystalEntity, GammaRateComponent>,
    dt: f64,
) {
    for(entity, gamma_component) in gamma_map.iter_mut(){
        if let Some(gamma_rate_component) = gamma_rate_map.get(entity){
            for index in 0..24 {
                let value = gamma_rate_component.get_values(index).expect("Ошибка извлечения gamma") * dt;
                gamma_component.set_values(index, value);
            }
        } else {
            panic!("Ошибка поиска компоненты gamma_rate")
        }
    }
}

pub fn calc_gamma_rate(
    gamma_rate_map: &mut HashMap<CrystalEntity, GammaRateComponent>,
    tau_map: &HashMap<CrystalEntity, TauComponent>,
    tau_c_map: &HashMap<CrystalEntity, TauComponent>,
    gamma_0: f64,
    m: f64,
) {
    for (entity, gamma_rate_component) in gamma_rate_map.iter_mut() {
        if let Some(tau_component) = tau_map.get(entity) {
            if let Some(tau_c_component) = tau_c_map.get(entity) {
                for index in 0..24 {
                    let tau = tau_component
                        .get_values(index)
                        .expect("Ошибка извлечения tau");
                    let tau_c = tau_c_component
                        .get_values(index)
                        .expect("Ошибка извлечения tau_c");
                    let ratio = tau / tau_c;
                    let gamma_rate = if ratio > 1.0 {
                        gamma_0 * ratio.powf(m)
                    } else {
                        0.0
                    };
                    gamma_rate_component.set_values(index, gamma_rate);
                }
            } else{
                panic!("Ошибка поиска компонента tau_c");
            }
        } else{
            panic!("Ошибка поиска компонента tau");
        }
    }
}

pub fn calc_tauc_rate_sat_law(
    tau_c_rate_map: &mut HashMap<CrystalEntity, TauRateComponent>,
    h_matrix_map: & HashMap<CrystalEntity, HMatrixComponent>,
    gamma_rate_map: &HashMap<CrystalEntity, GammaRateComponent>,
){
    for (entity, tauc_rate_component) in tau_c_rate_map.iter_mut(){
        if let Some(h_matrix_component) = h_matrix_map.get(entity){
            if let Some(gamma_rate_component) = gamma_rate_map.get(entity){
                for index_k in 0..24{
                    let mut sum = 0.0;
                    for index_j in 0..24{
                        let h_matrix = h_matrix_component.get_value(index_k, index_j).expect("Ошибка извлечения h_matrix_component");
                        let gamma_rate = gamma_rate_component.get_values(index_j).expect("Ошибка извлечения gamma_rate");
                        sum+=h_matrix*gamma_rate;
                    }
                    let gamma_rate = gamma_rate_component.get_values(index_k).expect("Ошибка извлечения gamma_rate");
                    if gamma_rate.abs()>1e-7{
                        tauc_rate_component.set_values(index_k, sum);
                    } else {
                        tauc_rate_component.set_values(index_k, 0.0);
                    }
                }
            } else{
                panic!("Ошибка поиска gamma_rate")
            }
        } else{
            panic!("Ошибка поиска h_matrix")
        }
    }
}

pub fn calc_tauc(
    tau_c_map: &mut HashMap<CrystalEntity, TauComponent>,
    tau_c_rate_map: &mut HashMap<CrystalEntity, TauRateComponent>,
    dt:f64,
){
    for (entity,tauc_component) in tau_c_map.iter_mut(){
        if let Some(tauc_rate_component) = tau_c_rate_map.get(entity){
            for index in 0..24{
                let tauc_rate = tauc_rate_component.get_values(index).expect("Ошибка извлечения tauc_rate");
                let tauc = tauc_component.get_values(index).expect("Ошибка извлечения tauc");
                let value = tauc+tauc_rate*dt;
                tauc_component.set_values(index, value);
            }
        }
    }
}

pub fn calc_h_vector(
    h_vector_map:  &mut HashMap<CrystalEntity, HVectorComponent>,
    tau_c_map: &HashMap<CrystalEntity, TauComponent>,
    tau_sat: f64,
    h0: f64,
    a: f64,
){
    for (entity, h_vector_component) in h_vector_map.iter_mut(){
        if let Some(tauc_component) = tau_c_map.get(entity){
            for index in 0..24{
                let tauc = tauc_component
                    .get_values(index)
                    .expect("Ошибка извлечения tau_c");
                let tau_sat_mpa=tau_sat*1e-6;
                let ratio = tauc/tau_sat_mpa;
                let absol = (1.0-ratio).abs();
                
                let pow_absol = absol.powf(a);
                
                let value = h0*pow_absol;
                h_vector_component.set_vector(index, value);
            }
        } else{
            panic!("Ошибка поиска tauc");
        }
    }
}

pub fn calc_h_matrix(
    h_matrix_map: &mut HashMap<CrystalEntity, HMatrixComponent>,
    h_vector_map: &HashMap<CrystalEntity, HVectorComponent>,
    qlat:f64,
){
    for (entity, h_matrix_component) in h_matrix_map.iter_mut(){
        if let Some(h_vector_component) = h_vector_map.get(entity){
            for index_i in 0..24{
                for index_j in 0..24{
                    let h_vector_j = h_vector_component.get_value(index_j).expect("Ошибка извлечения h_vector_i");
                    let multiply = if index_i==index_j {1.0} else{qlat};
                    let value = h_vector_j* multiply; 
                    h_matrix_component.set_value(index_i, index_j, value)
                }
            }
        }
    }
}