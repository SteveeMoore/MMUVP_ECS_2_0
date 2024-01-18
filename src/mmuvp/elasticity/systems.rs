#![allow(dead_code)]
use rayon::prelude::*;

use std::{
    collections::HashMap, 
    fs::{File, OpenOptions}, 
    io::{self, BufRead, BufWriter, Write}, 
    path::PathBuf};

use nalgebra::{Matrix3, Matrix6, Vector6};

use crate::{mmuvp::{
    entity::CrystalEntity, 
    rotation::components::*, 
    slide_system::components::*
}, consts::{FILE_INPUT_PATH, FILE_OUTPUT_PATH, MEGA}};

use super::components::*;

pub fn initialize_grad_v(
    grad_v_map: &mut HashMap<CrystalEntity, GradVComponent>,
    rotation_map: &HashMap<CrystalEntity, RotationComponent>,
    init_grad_v: Matrix3<f64>,
) {
    let mut new_matrix: Matrix3<f64>;
    for (entity, grad_v_component) in grad_v_map.iter_mut() {
        if let Some(orient_component) = rotation_map.get(entity) {
            new_matrix = orient_component.get_tensor().transpose()
                * init_grad_v
                * orient_component.get_tensor();
        } else {
            panic!("Ошибка поиска тензора ориентаций");
        }

        grad_v_component.set_tensor(new_matrix);
    }
}

pub fn read_grad_v_from_file_with_6_comp(
    trajectory_deformation: &mut Vec<TrajectoryDeformationComponent>,
) 
{
    let file = File::open(PathBuf::from(FILE_INPUT_PATH).join("grad_v.input")).expect("Не удалось открыть файл grad_v.input");
    let reader = io::BufReader::new(file);

    //let mut tensor_data_list = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Потеряна строка в файле grad_v.input");
        let mut values = line.split_whitespace();

        let time: f64 = values.next()
        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Missing time value"))
        .unwrap()
        .parse().unwrap();
        
        let mut components = [0.0; 6];
        for component in components.iter_mut() {
            *component = values
                .next()
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Missing tensor component"))
                .unwrap()
                .parse().unwrap();
        }
        let grad_v:Matrix3<f64> = Matrix3::new(
            components[0], components[1], components[2],
            components[1], components[3], components[4],
            components[2], components[4], components[5]
        );
        let mut deformation = TrajectoryDeformationComponent::new();
        println!("{:?}",grad_v);
        deformation.set_value(time, grad_v);
        trajectory_deformation.push(deformation);
    }
}

pub fn initialize_d(
    d_map: &mut HashMap<CrystalEntity, DComponent>,
    grad_v_map: & HashMap<CrystalEntity, GradVComponent>
) {
    for (entity, d_component) in d_map.iter_mut(){
        if let Some(grad_v_component) = grad_v_map.get(entity){
            let grad_v = grad_v_component.get_tensor();
            let tensor = (grad_v+grad_v.transpose())/2.0;
            d_component.set_tensor(tensor);
        } else {
            panic!("Ошибка поиска grad_v")
        }
    }
}

pub fn initialize_w(
    w_map: &mut HashMap<CrystalEntity, WComponent>,
    grad_v_map: & HashMap<CrystalEntity, GradVComponent>,
){
    for (entity, w_component) in w_map.iter_mut(){
        if let Some(grad_v_component) = grad_v_map.get(entity){
            let grad_v = grad_v_component.get_tensor();
            let tensor = (grad_v.transpose()-grad_v)/2.0;
            w_component.set_tensor(tensor);
        }else {
            panic!("Ошибка поиска w")
        }
    }
}

pub fn calc_eps(
    eps_map: &mut HashMap<CrystalEntity, EpsComponent>,
    d_map: &HashMap<CrystalEntity, DComponent>,
    dt: f64,
) {
    //for (entity, e_component) in eps_map.iter_mut() {
    eps_map.par_iter_mut().for_each(|(entity,e_component)|{
        if let Some(d_component) = d_map.get(entity) {
            let mut summ = Matrix3::zeros();
            summ += e_component.get_tensor();
            summ += d_component.get_tensor() * dt;
            e_component.set_tensor(summ);
        } else {
            panic!("Ошибка поиска компонента деформации скорости");
        }
    })
}

pub fn calc_mean_eps(
    eps_map: &HashMap<CrystalEntity, EpsComponent>,
    rotation_map: &HashMap<CrystalEntity, RotationComponent>,
) -> Matrix3<f64> {
    let mut mean_matrix = Matrix3::zeros();
    for (entity, e_component) in eps_map.iter() {
        if let Some(orient_component) = rotation_map.get(entity) {
            mean_matrix += orient_component.get_tensor()
                * e_component.get_tensor()
                * orient_component.get_tensor().transpose();
        } else {
            panic!("Ошибка поиска тензора ориентации");
        }
    }
    (1.0 / eps_map.len() as f64) * mean_matrix
}

pub fn calc_intensity_eps(din_component: &EpsComponent) -> f64 {
    let din = din_component.get_tensor();
    (din.dot(&din) * 2.0 / 3.0).sqrt()
}

pub fn calc_sigma(
    sigma_map: &mut HashMap<CrystalEntity, SigmaComponent>,
    sigma_rate_map: &HashMap<CrystalEntity, SigmaRateComponent>,
    dt: f64,
) {
    //for (entity, sigma_component) in sigma_map.iter_mut() {
    sigma_map.par_iter_mut().for_each(|(entity,sigma_component)|{
        if let Some(sigma_rate_component) = sigma_rate_map.get(entity) {
            let mut summ = Matrix3::zeros();
            summ += sigma_component.get_tensor();
            summ += sigma_rate_component.get_tensor() * dt;
            sigma_component.set_tensor(summ);
        } else {
            panic!("Ошибка поиска компонента sigma");
        }
    })
}

pub fn calc_mean_sigma(
    sigma_map: &HashMap<CrystalEntity, SigmaComponent>,
    rotation_map: &HashMap<CrystalEntity, RotationComponent>,
) -> Matrix3<f64> {
    let mut mean_matrix = Matrix3::zeros();
    for (entity, sigma_component) in sigma_map.iter() {
        if let Some(orient_component) = rotation_map.get(entity) {
            mean_matrix += orient_component.get_tensor()
                * sigma_component.get_tensor()
                * orient_component.get_tensor().transpose();
        } else {
            panic!("Ошибка поиска тензора ориентации");
        }
    }
    (1.0 / sigma_map.len() as f64) * mean_matrix
}

pub fn calc_intensity_s(sigma_component: &SigmaComponent) -> f64 {
    let sigma = sigma_component.get_tensor();
    (sigma.dot(&sigma) * 3.0 / 2.0).sqrt()
}

pub fn write_intensity_to_file(
    polycrystal_eps: &EpsComponent,
    polycrystal_sigma: &SigmaComponent,
    step: i64,
    dt: f64,
) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("rvout.dat"))
        .expect("Ошибка открытия файла rvout.dat");
    //let file = File::create(FILE_OUTPUT_PATH.to_string() + "din.dat")?;
    let mut buf_writer = BufWriter::with_capacity(4 * (10 + 1 + 10 + 1) * 3, file);

    write!(buf_writer, "{:.4e}\t", calc_intensity_eps(polycrystal_eps))
        .expect("Ошибка записи интенсивности деформации в rvout.dat");
    write!(buf_writer, "{:.4e}\t", calc_intensity_s(polycrystal_sigma))
        .expect("Ошибка записи интенсивности напряжения в rvout.dat");
    write!(buf_writer, "{}\t", dt * step as f64).expect("Ошибка записи времени в rvout.dat");

    writeln!(buf_writer).expect("Ошибка записи разделителя в rvout.dat");

    // Завершаем запись и проверяем наличие ошибок
    buf_writer
        .flush()
        .expect("Ошибка завершения записи в rvout.dat");
}

pub fn calc_de_elastic_deform(
    de_map: &mut HashMap<CrystalEntity, DComponent>,
    d_map: &HashMap<CrystalEntity, DComponent>,
) {
    for (entity, de_component) in de_map.iter_mut() {
        if let Some(d_component) = d_map.get(entity) {
            let d = d_component.get_tensor();
            de_component.set_tensor(d);
        }
    }
}

pub fn calc_de_elastic_plastic_deform(
    de_map: &mut HashMap<CrystalEntity, DComponent>,
    d_map: &HashMap<CrystalEntity, DComponent>,
    din_map: &HashMap<CrystalEntity, DComponent>,
) {
    for (entity, de_component) in de_map.iter_mut() {
        if let Some(d_component) = d_map.get(entity) {
            let d = d_component.get_tensor();
            if let Some(din_component) = din_map.get(entity) {
                let din = din_component.get_tensor();
                de_component.set_tensor(d - din);
            }
        }
    }
}

pub fn initialize_elasticity_tensor_fcc(
    elasticity_map: &mut HashMap<CrystalEntity, ElasticityTensorComponent>,
    c11: f64,
    c12: f64,
    c44: f64,
    koef:f64,
) {
    let c11 = c11 / MEGA;//MPa
    let c12 = c12 / MEGA;//MPa
    let c44 = c44 / MEGA;//MPa
    for c_tensor in elasticity_map.values_mut() {
        let value = Matrix6::new(
            c11, c12, c12, 0.0, 0.0, 0.0, c12, c11, c12, 0.0, 0.0, 0.0, c12, c12, c11, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, c44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, c44, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, c44,
        )*koef;
        c_tensor.set_value(value);
    }
}

pub fn calc_hooke_law(
    sigma_rate_map: &mut HashMap<CrystalEntity, SigmaRateComponent>,
    elasticity_map: &HashMap<CrystalEntity, ElasticityTensorComponent>,
    de_map: &HashMap<CrystalEntity, DComponent>,
) {
    for (entity, sigma_rate_component) in sigma_rate_map.iter_mut() {
        if let Some(elasticity_tensor_component) = elasticity_map.get(entity) {
            let c = elasticity_tensor_component.get_value();
            if let Some(de_component) = de_map.get(entity) {
                let de: Vector6<f64> = de_component.get_vector();
                let value: Vector6<f64> = c * de;
                sigma_rate_component.set_vector(value);
            } else {
                panic!("Ошибка поиска компанента de");
            }
        }
    }
}

pub fn calc_din(
    din_map: &mut HashMap<CrystalEntity, DComponent>,
    gamma_rate_map: &HashMap<CrystalEntity, GammaRateComponent>,
    bn_map: &HashMap<CrystalEntity, BNComponent>,
) {
    for (entity, din_component) in din_map.iter_mut() {
        if let Some(gamma_rate_component) = gamma_rate_map.get(entity) {
            if let Some(bn_component) = bn_map.get(entity) {
                let mut summ = Matrix3::zeros();
                for index in 0..24 {
                    let gamma_rate = gamma_rate_component
                        .get_values(index)
                        .expect("Не удалось найти gamma_rate");
                    let bn = bn_component
                        .get_matrix(index)
                        .expect("Не удалось найти матрицу bn");
                    let term = gamma_rate * bn;
                    summ += term;
                }
                din_component.set_tensor(summ);
            } else {
                panic!("Ошибка поиска компонента bn");
            }
        } else {
            panic!("Ошибка поиска компонента gamma_rate");
        }
    }
}