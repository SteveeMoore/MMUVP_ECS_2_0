#![allow(dead_code)]

use std::collections::HashMap;

use crate::{
    mmuvp::{entity::CrystalEntity, 
        elasticity::components::*
    }, 
    consts::MEGA
};

use super::components::*;
use rand::{Rng,
    distributions::Distribution};
use statrs::distribution::LogNormal;

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
                let sigma_tensor = sigma_component.get_tensor()*MEGA;
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
    r0: f64,
    num: usize,
)
{
     // Вектор для хранения значений из файла
     let mut values: Vec<f64> = Vec::new();

    get_distr_rayleigh(&mut values, num, r0);

    println!("{:?}",values);
    for value in values{
        for subgrains_component in subgrains_map.values_mut(){
            subgrains_component.push_value(value);
        }
     }
}

pub fn get_distr_rayleigh(distr: &mut Vec<f64>, num: usize, r0: f64) {
    let mut ev_dist = Vec::new();
    let mut distr_den = Vec::new();
    let mut rando;
    let mut max:f64;
    let mut nnum = num;
    let mut rng = rand::thread_rng();
    loop {
        nnum += num / 2;
        ev_dist.clear();
        distr_den.clear();
        distr.clear();
        max = 0.0;

        for _ in 0..nnum {
            rando = rng.gen_range(0.0..(10.0 * r0));
            ev_dist.push(rando);
        }

        #[allow(clippy::needless_range_loop)]
        for i in 0..nnum {
            rando = distr_dens_rayleigh(r0, ev_dist[i]);
            distr_den.push(rando);
            if rando > max {
                max = rando;
            }
        }

        for i in 0..nnum {
            if distr_den[i] / max >= rng.gen_range(0.0..1.0) {
                distr.push(ev_dist[i]);
            }
        }

        if distr.len() >= num {
            break;
        }
    }

    nnum = distr.len() - num;
    distr.drain(..nnum);
}

pub fn distr_dens_rayleigh(r0: f64, x: f64) -> f64 {
    let sigma = r0 * f64::sqrt(2.0 / (2.0 * f64::asin(1.0)));
    if x >= 0.0 {
        x / (sigma * sigma) * f64::exp(-(x * x) / (2.0 * sigma * sigma))
    } else {
        0.0
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
    status_map: &HashMap<CrystalEntity, StatusRecrystComponent>,
    subgrains_map:& HashMap<CrystalEntity, SubGrainsComponent>,
    est_poly_component: &AccumEnergyComponent,
    egb:f64,
)
{
    for (entity, df_recr_component) in df_recr_map.iter_mut(){
        if let Some(status_component) = status_map.get(entity){
            if status_component.get_value(){
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

pub fn init_grain_size(
    gr_size_map: &mut HashMap<CrystalEntity, GrainSizeComponent>,
    mean:f64,
    std_dev: f64, 
){
    for gr_size_component in gr_size_map.values_mut(){
        let value = generate_lognormal_random_number(mean, std_dev);
        gr_size_component.set_value(value);
    }
}

fn generate_lognormal_random_number(mean: f64, std_dev: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let lognormal = LogNormal::new(mean, std_dev).unwrap();
    lognormal.sample(&mut rng)
}

