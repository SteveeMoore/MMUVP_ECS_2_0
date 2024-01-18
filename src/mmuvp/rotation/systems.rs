#![allow(dead_code)]
use nalgebra::{Matrix3, Vector3};
use rand::Rng;
use std::{
    collections::HashMap,
    f64::consts::PI,
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::{consts::FILE_OUTPUT_PATH, mmuvp::entity::CrystalEntity};

use super::components::RotationComponent;

pub fn gen_uniform_distribution(rotation_map: &mut HashMap<CrystalEntity, RotationComponent>) {
    if rotation_map.len() > 1 {
        for rotation in rotation_map.values_mut() {
            let matrix = get_uniform_distribution();
            rotation.set_matrix(matrix).unwrap();
        }
    }
}

pub fn get_uniform_distribution()->Matrix3<f64>{
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0.0..2.0 * PI);
    let b: f64 = rng.gen_range(-1.0..1.0);
    let b = b.acos();
    let g = rng.gen_range(0.0..2.0 * PI);
    let ca = a.cos();
    let sa = a.sin();
    let cb = b.cos();
    let sb = b.sin();
    let cg = g.cos();
    let sg = g.sin();

    Matrix3::new(
        ca * cb * cg - sa * sg,
        -cg * sa - ca * cb * sg,
        ca * sb,
        cb * cg * sa + ca * sg,
        ca * cg - cb * sa * sg,
        sa * sb,
        -cg * sb,
        sb * sg,
        cb,
    )
}

pub fn write_pole_figure(rotation_map: &HashMap<CrystalEntity, RotationComponent>) {
    let file100 = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("pole_fig100.dat"))
        .expect("Ошибка открытия файла для записи poly_fig100.dat");
    //let file100 = File::create(FILE_OUTPUT_PATH.to_string() + "pole_fig100.dat")?;
    let mut buf_writer100 = BufWriter::with_capacity(4 * 25 * 3 * rotation_map.len() + 4, file100);

    let file110 = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("pole_fig110.dat"))
        .expect("Ошибка открытия файла для записи pole_fig110.dat");
    //let file110 = File::create(FILE_OUTPUT_PATH.to_string() + "pole_fig110.dat")?;
    let mut buf_writer110 = BufWriter::with_capacity(4 * 25 * 3 * rotation_map.len() + 4, file110);

    let file111 = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("pole_fig111.dat"))
        .expect("Ошибка открытия файла для записи pole_fig111.dat");
    //let file111 = File::create(FILE_OUTPUT_PATH.to_string() + "pole_fig111.dat")?;
    let mut buf_writer111 = BufWriter::with_capacity(4 * 25 * 3 * rotation_map.len() + 4, file111);

    for rotation in rotation_map.values() {
        let test_vector100 = Vector3::new(1.0, 0.0, 0.0);
        let rotation_vector100 =
            (test_vector100.normalize().transpose() * rotation.get_tensor()).transpose();
        let test_vector110 = Vector3::new(1.0, 1.0, 0.0);
        let rotation_vector110 =
            (test_vector110.normalize().transpose() * rotation.get_tensor()).transpose();
        let test_vector111 = Vector3::new(1.0, 1.0, 1.0);
        let rotation_vector111 =
            (test_vector111.normalize().transpose() * rotation.get_tensor()).transpose();

        write!(
            buf_writer100,
            "{}\t{}\t{}\t",
            rotation_vector100.x, rotation_vector100.y, rotation_vector100.z
        )
        .expect("Ошибка записи полюсных фигур 100");
        write!(
            buf_writer110,
            "{}\t{}\t{}\t",
            rotation_vector110.x, rotation_vector110.y, rotation_vector110.z
        )
        .expect("Ошибка записи полюсных фигур 110");
        write!(
            buf_writer111,
            "{}\t{}\t{}\t",
            rotation_vector111.x, rotation_vector111.y, rotation_vector111.z
        )
        .expect("Ошибка записи полюсных фигур 111");
    }

    writeln!(buf_writer100).expect("Ошибка записи разделителя полюсных фигур 100");
    writeln!(buf_writer110).expect("Ошибка записи разделителя полюсных фигур 110");
    writeln!(buf_writer111).expect("Ошибка записи разделителя полюсных фигур 111");

    buf_writer100
        .flush()
        .expect("Ошибка завершения записи полюсных фигур 100");
    buf_writer110
        .flush()
        .expect("Ошибка завершения записи полюсных фигур 110");
    buf_writer111
        .flush()
        .expect("Ошибка завершения записи полюсных фигур 111");
}

pub fn write_rotation_to_file(rotation_map: &HashMap<CrystalEntity, RotationComponent>) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PathBuf::from(FILE_OUTPUT_PATH).join("orient.dat"))
        .expect("Ошибка открытия файла orient.dat");

    let mut buf_writer = BufWriter::with_capacity(4 * 25 * 9 * rotation_map.len() + 4, file);

    // Записываем данные в файл
    for rotation in rotation_map.values() {
        let tensor_orient = rotation.get_tensor();
        for o_i in tensor_orient.iter() {
            write!(buf_writer, "{}\t", *o_i).expect("Ошибка записи тензора ориентации");
        }
    }
    writeln!(buf_writer).expect("Ошибка записи разделителя в тензоре ориентации");

    // Завершаем запись и проверяем наличие ошибок
    buf_writer
        .flush()
        .expect("Ошибка завершения записи тензора ориентации");
}
/*
pub fn calc_spin(
    spin_map: &mut HashMap<CrystalEntity, SpinComponent>,
    w_map: &HashMap<CrystalEntity,WComponent>,
    bn_map: &HashMap<CrystalEntity,BNComponent>,
    gamma_rate_map: &HashMap<CrystalEntity,GammaRateComponent>
)
{
    for (entity, spin) in spin_map.iter_mut(){
        if let Some(w_component)= w_map.get(entity) {
            if let Some(bn_component)= bn_map.get(entity) {
                if let Some(gamma_component)= gamma_rate_map.get(entity) {
                    let w_tensor = w_component.get_tensor();
                    let mut second_term = Matrix3::zeros();
                    for index in 0..24 {
                        let bn = bn_component.get_matrix(index).expect("Ошибка извлечения bn");
                        let transpose_bn = bn.transpose();
                        let gamma_rate = gamma_component.get_values(index).expect("Ошибка извлечения gamma_rate");
                        second_term += gamma_rate*(bn-transpose_bn);
                    }
                    second_term/=2.0;
                    //println!("{:?}", second_term);
                    let tensor = w_tensor-second_term;
                    spin.set_tensor(tensor).unwrap();                    
                } 
                else {
                    panic!("Ошибка поиска компонента gamma_rate");
                }
            } 
            else {
                panic!("Ошибка поиска компонента bn");
            }
        } else {
            panic!("Ошибка поиска компонента grad_v");
        }
    }
    
}

pub fn calc_rotation_rate(
    rotation_rate_map: &mut HashMap<CrystalEntity,RotationRateComponent>,
    spin_map:&HashMap<CrystalEntity,SpinComponent>,
    dt:f64,
){
    let mut axis_angle:Vector3<f64> = Vector3::new(0.0, 0.0,0.0);

    for (entity, rotation_rate_component) in rotation_rate_map.iter_mut(){
        if let Some(spin_component) = spin_map.get(entity){
            let spin = spin_component.get_tensor();
            for i in 0..3{
                for j in 0..3{
                    for k in 0..3{
                        axis_angle[i] += levi_civita(i, j, k) * spin[(k,j)];
                    }
                }
            }
            let rotation = Rotation3::new(axis_angle*dt);
            let tensor = *rotation.matrix();
            rotation_rate_component.set_tensor(tensor).unwrap();
        } else {
            panic!("Ошибка поиска компонента spin")
        }
    }
    
}

pub fn calc_rotation(
    rotation_map:&mut HashMap<CrystalEntity,RotationComponent>,
    grad_v_map: &mut HashMap<CrystalEntity, GradVComponent>,
    d_map: &mut HashMap<CrystalEntity, DComponent>,
    w_map: &mut HashMap<CrystalEntity, WComponent>,
    rotation_rate_map: & HashMap<CrystalEntity,RotationRateComponent>,
    init_grad_v: Matrix3<f64>,
    dt:f64,
){
    for (entity, rotation_rate_component) in rotation_rate_map.iter() {
        if let Some(rotation_component) = rotation_map.get_mut(entity){
            let rotation = rotation_component.get_tensor();
            let rotation_rate = rotation_rate_component.get_tensor();
            let matrix = rotation_rate*rotation;
            rotation_component.set_matrix(matrix).unwrap();
        }
    }

    initialize_grad_v(grad_v_map, rotation_map, init_grad_v);
    initialize_d(d_map, grad_v_map);
    initialize_w(w_map, grad_v_map);
}

pub fn levi_civita(i:usize, j:usize, k:usize) -> f64 {
    if (i == j) || (j == k) || (k == i) {
        0.0
    } else if (i + 1) % 3 == j && (j + 1) % 3 == k {
        1.0
    } else if (i + 1) % 3 == k && (k + 1) % 3 == j {
        -1.0
    } else {
        0.0
    }
}
 */