#![allow(dead_code)]

use std::{collections::HashMap, fs::File, path::PathBuf, io::{BufReader, BufRead}};

use crate::{mmuvp::{entity::CrystalEntity, elasticity::components::{SigmaComponent, DComponent}}, consts::FILE_INPUT_PATH};

use super::components::*;

pub fn calc_accum_energy_rate(
    est_rate_map: &mut HashMap<CrystalEntity, AccumEnergyRateComponent>,
    sigma_map: &HashMap<CrystalEntity,SigmaComponent>,
    din_map:&HashMap<CrystalEntity,DComponent>,
    alfa:f64,
)
{
    for (entity, est_rate_component) in est_rate_map.iter_mut(){
        if let Some(sigma_component) = sigma_map.get(entity){
            if let Some(din_component) = din_map.get(entity){
                let sigma_tensor = sigma_component.get_tensor();
                let din_tensor = din_component.get_tensor();
                let value = alfa*(sigma_tensor.dot(&din_tensor));
                est_rate_component.set_value(value);
            } else{
                panic!("Ошибка поиска компонента din")
            }
        } else{
            panic!("Ошибка поиска компонента sigma")
        }
    }
}

pub fn calc_accum_energy(
    est_map:&mut HashMap<CrystalEntity,AccumEnergyComponent>,
    est_rate_map: & HashMap<CrystalEntity, AccumEnergyRateComponent>,
    dt:f64,
){
    for (entity, est_component) in est_map.iter_mut(){
        if let Some(est_rate_component) = est_rate_map.get(entity){
            let est_rate = est_rate_component.get_value();
            let est = est_component.get_value();
            let value = est + est_rate*dt;
            est_component.set_value(value);
        } else{
            panic!("Ошибка поиска компонента est_rate")
        }
    }
}

pub fn calc_mean_accum_energy(
    est_map:&HashMap<CrystalEntity,AccumEnergyComponent>,
)-> f64 {
    let mut value = 0.0;
    for est_component in est_map.values(){
        let est = est_component.get_value();
        value +=est;
    } 
    (1.0 / est_map.len() as f64) * value
}

pub fn initialize_subgrains(
    subgrains_map:&mut HashMap<CrystalEntity, SubGrainsComponent>,
)
{
    let file = File::open(PathBuf::from(FILE_INPUT_PATH).join("r.input")).expect("Ошибка открытия файла b.input");
    let reader = BufReader::new(file);

     // Вектор для хранения значений из файла
     let mut values: Vec<f64> = Vec::new();

     // Чтение строк из файла и запись значений в вектор
     for line in reader.lines() {
         if let Ok(value_str) = line {
             if let Ok(value) = value_str.trim().parse::<f64>() {
                 values.push(value);
             } else {
                 eprintln!("Ошибка преобразования строки в число: {}", value_str);
             }
         } else {
             eprintln!("Ошибка чтения строки из файла");
         }
     }

     
    for value in values{
        for subgrains_component in subgrains_map.values_mut(){
            subgrains_component.push_value(value);
        }
     }
}

pub fn initialize_drive_force_recr(
    df_recr_map: &mut HashMap<CrystalEntity, DriveForceRecrComponent>,
    subgrains_map:& HashMap<CrystalEntity, SubGrainsComponent>
){
    for df_recr_component in df_recr_map.values_mut(){
        for _index in 0..subgrains_map.len(){
            df_recr_component.push_value(0.0);
        }
    }
}

pub fn calc_drive_force_recr(
    df_recr_map: &mut HashMap<CrystalEntity, DriveForceRecrComponent>,
    subgrains_map:& HashMap<CrystalEntity, SubGrainsComponent>,
    est_poly_component: &AccumEnergyComponent,
    egb:f64,
)
{
    for (entity, df_recr_component) in df_recr_map.iter_mut(){
        if let Some(subgrains_component) = subgrains_map.get(entity){
            for index in 0..df_recr_component.len(){
                let subgrains_r = subgrains_component.get_value(index).unwrap();
                let est_poly = est_poly_component.get_value();
                let value = est_poly - 3.0 * egb / subgrains_r;
                df_recr_component.set_value(index, value);
            }
        } else {
            panic!("Ошибка поиска компонента subgrains")
        }
    }
}

pub fn calc_drive_force_recr_cryst(
    df_recr_cryst_map: &mut HashMap<CrystalEntity, DriveForceRecrCrystComponent>,
    gr_size_map:&HashMap<CrystalEntity,GrainSizeComponent>,
    est_poly_component: &AccumEnergyComponent,
    egb:f64
){
    for (entity, df_recr_cryst_component) in df_recr_cryst_map.iter_mut(){
        if let Some(gr_size_component) = gr_size_map.get(entity){
            let gr_size = gr_size_component.get_value();
            let est_poly = est_poly_component.get_value();
            let value = est_poly - 3.0 * egb / gr_size;
            df_recr_cryst_component.set_value(value);
        }else {
            panic!("Ошибка поиска компонента gr_size")
        }
    }
}

pub fn calc_facet_mobility(
    facet_mobility_map: &mut HashMap<CrystalEntity, FacetMobilityComponent>,
    m0:f64,
    q:f64,
    r:f64,
    temp:f64,
) {
    for facet_mobility_component in facet_mobility_map.values_mut(){
        let value = m0 * (-q/(r*temp)).exp();
        facet_mobility_component.set_value(value);
    }
}

pub fn calc_vel_facet(
    vel_facet_map:&mut HashMap<CrystalEntity,VelocityFacetComponent>,
    df_recr_cryst_map: & HashMap<CrystalEntity, DriveForceRecrCrystComponent>,
    facet_mobility_map: & HashMap<CrystalEntity, FacetMobilityComponent>
){
    for (entity, vel_facet_component) in vel_facet_map.iter_mut(){
        if let Some(df_recr_cryst_component) = df_recr_cryst_map.get(entity){
            if let Some(facet_mobility_component) = facet_mobility_map.get(entity){
                let df_recr_cryst = df_recr_cryst_component.get_value();
                let facet_mobility = facet_mobility_component.get_value();
                let value = df_recr_cryst * facet_mobility;
                vel_facet_component.set_value(value);
            } else {
                panic!("Ошибка поиска компонента facet_mobility")
            }
        } else {
            panic!("Ошибка поиска компонента df_rect_cryst")
        }
    }
}

pub fn calc_grain_size(
    gr_size_map:&mut HashMap<CrystalEntity,GrainSizeComponent>,
    vel_facet_map:& HashMap<CrystalEntity,VelocityFacetComponent>,
    dt:f64
){
    for (entity, gr_size_component) in gr_size_map.iter_mut(){
        if let Some(vel_facet_component) = vel_facet_map.get(entity){
            let vel_facet = vel_facet_component.get_value();
            let gr_size = gr_size_component.get_value();
            let value = gr_size+vel_facet*dt;
            gr_size_component.set_value(value);
        }
    }
}