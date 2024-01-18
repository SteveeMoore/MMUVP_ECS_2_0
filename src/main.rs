//Ниже добавлены все необходимые модули
mod mmuvp;
mod consts;
mod base_fn;

//Ниже добавлены все системные функции
use std::{collections::HashMap, time::Instant};

//Ниже добавлены все используемые компоненты и системы. Вручную желательно не исправлять во избежание ошибок.
use base_fn::*;

use mmuvp::{
    params::{
        components::Params, 
        systems::from_file
    }, 
    standart_deformation::*,
    entity::CrystalEntity, 
    rotation::{
        components::*,
        systems::*
    }, 
    elasticity::{
        components::*, 
        systems::*
    },
    slide_system::{
        components::*,
        systems::*
    }, 
    recrystallization::{
        components::*,
        systems::*
    }
};

use crate::mmuvp::recrystallization::systems::init_grain_size;




//Начало основной программы
fn main() {
    //Очищение файлы в папке output. 
    clear_output_folder();

    //Объявление и инициализация HashMap где будут храниться все параметры модели
    let mut params = Params::new();
    //Считываем все параметры из файла param.json
    from_file(&mut params);
    //Для удобства некоторые часто используемые параметры можно записать в отдельные переменные
    let dt = params.get_f64("dt");

    //Ниже записываются начальные условия, считывается траектория деформирования или задается деформации(гипотеза Фойгта)/напряжения(гипотеза Рейса).
    let init_grad_v = uniaxial_tension(1.0e-2);
    

    //Ниже объявляются HashMap всех компонентов. Первый аргумент - название переменной, второй - сущность, третий - компонент. 
    //Рекомендуется использовать стандартные переменные
    create_component_map!(rotation_map, CrystalEntity, RotationComponent);
    create_component_map!(grad_v_map, CrystalEntity, GradVComponent);
    create_component_map!(d_map, CrystalEntity, DComponent);
    create_component_map!(de_map, CrystalEntity, DComponent);
    create_component_map!(din_map, CrystalEntity, DComponent);
    create_component_map!(sigma_map, CrystalEntity, SigmaComponent);
    create_component_map!(sigma_rate_map, CrystalEntity, SigmaRateComponent);
    create_component_map!(elasticity_map, CrystalEntity, ElasticityTensorComponent);
    create_component_map!(eps_map, CrystalEntity, EpsComponent);
    create_component_map!(burgers_map,CrystalEntity, BurgersVectorComponent);
    create_component_map!(normals_map, CrystalEntity, NormalVectorComponent);
    create_component_map!(bn_map, CrystalEntity, BNComponent);
    create_component_map!(tau_map, CrystalEntity, TauComponent);
    create_component_map!(tau_c_map, CrystalEntity, TauComponent);
    create_component_map!(tau_rate_map, CrystalEntity, TauRateComponent);
    create_component_map!(tau_c_rate_map, CrystalEntity, TauRateComponent);
    create_component_map!(gamma_map, CrystalEntity, GammaComponent);
    create_component_map!(gamma_rate_map, CrystalEntity, GammaRateComponent);
    create_component_map!(h_vector_map, CrystalEntity, HVectorComponent);
    create_component_map!(h_matrix_map, CrystalEntity, HMatrixComponent);
    create_component_map!(gr_size_map,CrystalEntity, GrainSizeComponent);
    create_component_map!(est_map,CrystalEntity, AccumEnergyComponent);
    create_component_map!(est_rate_map, CrystalEntity, AccumEnergyRateComponent);
    create_component_map!(status_map, CrystalEntity, StatusRecrystComponent);
    create_component_map!(facet_mobility_map, CrystalEntity, FacetMobilityComponent);
    create_component_map!(subgrains_map,CrystalEntity,SubGrainsComponent);
    create_component_map!(df_recr_map,CrystalEntity, DriveForceRecrComponent);
    create_component_map!(df_recr_cryst_map, CrystalEntity, DriveForceRecrCrystComponent);
    create_component_map!(vel_facet_map, CrystalEntity, VelocityFacetComponent);


    //Ниже заполняются компоненты для каждого зерна в цикле
    for i in 0..params.get_i64("grain_num"){
        //Создается новый объект 
        let entity = CrystalEntity::new(i.try_into().unwrap());

        //Заполняются все HashMap объявленные выше. 
        //Первый аргумент - сущность, второй аргумент - новый экземпляр компонента, третий аргумент - HashMap соответствующего компонента
        insert_component!(entity, RotationComponent::new(), rotation_map);
        insert_component!(entity, GradVComponent::new(), grad_v_map);
        insert_component!(entity, DComponent::new(), d_map);
        insert_component!(entity, DComponent::new(), de_map);
        insert_component!(entity, DComponent::new(), din_map);
        insert_component!(entity, SigmaComponent::new(), sigma_map);
        insert_component!(entity, SigmaRateComponent::new(), sigma_rate_map);
        insert_component!(entity, ElasticityTensorComponent::new(), elasticity_map);
        insert_component!(entity, EpsComponent::new(), eps_map);
        insert_component!(entity, BurgersVectorComponent::new(), burgers_map);
        insert_component!(entity, NormalVectorComponent::new(), normals_map);
        insert_component!(entity, BNComponent::new(), bn_map);
        insert_component!(entity, TauComponent::new(), tau_map);
        insert_component!(entity, TauComponent::new(), tau_c_map);
        insert_component!(entity, TauRateComponent::new(), tau_rate_map);
        insert_component!(entity, TauRateComponent::new(), tau_c_rate_map);
        insert_component!(entity, GammaComponent::new(), gamma_map);
        insert_component!(entity, GammaRateComponent::new(), gamma_rate_map);
        insert_component!(entity, HVectorComponent::new(), h_vector_map);
        insert_component!(entity, HMatrixComponent::new(), h_matrix_map);
        insert_component!(entity, GrainSizeComponent::new(), gr_size_map);
        insert_component!(entity, AccumEnergyComponent::new(), est_map);
        insert_component!(entity, AccumEnergyRateComponent::new(), est_rate_map);
        insert_component!(entity, StatusRecrystComponent::new(), status_map);
        insert_component!(entity, FacetMobilityComponent::new(), facet_mobility_map);
        insert_component!(entity, SubGrainsComponent::new(), subgrains_map);
        insert_component!(entity, DriveForceRecrComponent::new(), df_recr_map);
        insert_component!(entity, DriveForceRecrCrystComponent::new(), df_recr_cryst_map);
        insert_component!(entity, VelocityFacetComponent::new(), vel_facet_map);
    }

    //Ниже инициализируются все необходимые переменные
    gen_uniform_distribution(&mut rotation_map);
    initialize_burgers_vectors(&mut burgers_map);
    initialize_normal_vectors(&mut normals_map);
    initialize_bn(&mut bn_map, &burgers_map, &normals_map);
    initialize_elasticity_tensor_fcc(&mut elasticity_map, params.get_f64("c11"), params.get_f64("c12"), params.get_f64("c44"), params.get_f64("koef"));
    //initialize_tau_c(&mut tau_c_map, params.get_f64("tau_c"));
    initialize_tau_c_hp(&mut tau_c_map, params.get_f64("tau_c"),params.get_f64("b"), params.get_f64("k_y"), params.get_f64("gr_size"), );
    initialize_grad_v(&mut grad_v_map, &rotation_map, init_grad_v);
    initialize_d(&mut d_map, &grad_v_map);
    init_grain_size(&mut gr_size_map, params.get_f64("gr_size"), params.get_f64("std_dev"));
    initialize_subgrains(&mut subgrains_map, params.get_f64("r0"), params.get_i64("num_sg") as usize);
    initialize_drive_force_recr(&mut df_recr_map, &subgrains_map);
    //Объявление и инциализация переменных для поликристалла
    let mut polycrystal_sigma = SigmaComponent::new();
    let mut polycrystal_eps = EpsComponent::new();
    let mut est_poly_component = AccumEnergyComponent::new();
    let new_grains = &mut NewGrainsComponent::new();
    
    //Ниже можно указать вывод данных которые необходимо вывести для отсчетной конфигурации
    write_pole_figure(&rotation_map);

    //Начало временного отсчета
    let time = Instant::now();

    //Начало расчета
    for step in 0..params.get_i64("steps_num"){
        initialize_grad_v(&mut grad_v_map, &rotation_map, init_grad_v);
        initialize_d(&mut d_map, &grad_v_map);
        //Вычисление текущего времени
        let current_time = time.elapsed();
        //Вычисление НДС для поликристалла, вывод интенсивностей в файл и вывод текущего состояния на экран.
        //При необходимости внутрь цикла можно добавлять вывод соответствующих значений, которые будут выводиться каждый write_step шагов
        if step % params.get_i64("write_step") == 0 {
            polycrystal_sigma.set_tensor(calc_mean_sigma(&sigma_map, &rotation_map));
            polycrystal_eps.set_tensor(calc_mean_eps(&eps_map, &rotation_map));
            write_intensity_to_file(&polycrystal_eps, &polycrystal_sigma, step, dt);
            print_current_sys(current_time, step, params.get_i64("steps_num"), &polycrystal_eps, &polycrystal_sigma);
        }
        //Вычисление всех компонент согласно выбранной модели. Если для компонент были выбраны стандартные имена переменных,
        //то аргументы функций заполняются автоматически, если в руководстве не сказано иное
        calc_tau(&mut tau_map, &bn_map, &sigma_map);
        calc_gamma_rate(&mut gamma_rate_map, &tau_map, &tau_c_map, params.get_f64("gamma_0"), params.get_f64("m"));
        calc_gamma(&mut gamma_map, &gamma_rate_map, dt);
        calc_h_vector(&mut h_vector_map, &tau_c_map, params.get_f64("tau_sat"), params.get_f64("h0"), params.get_f64("a"));
        calc_h_matrix(&mut h_matrix_map, &h_vector_map, params.get_f64("qlat"));
        calc_tauc_rate_sat_law(&mut tau_c_rate_map, &h_matrix_map, &gamma_rate_map);
        calc_tauc(&mut tau_c_map, &mut tau_c_rate_map, dt);
        calc_din(&mut din_map, &gamma_rate_map, &bn_map);
        calc_de_elastic_plastic_deform(&mut de_map, &d_map, &din_map);
        calc_hooke_law(&mut sigma_rate_map, &elasticity_map, &de_map);
        calc_sigma(&mut sigma_map, &sigma_rate_map, dt);
        calc_eps(&mut eps_map, &d_map, dt);
        calc_accum_energy_rate(&mut est_rate_map, &sigma_map, &din_map, params.get_f64("alfa"));
        calc_accum_energy(&mut est_map, &est_rate_map, dt);
        est_poly_component.set_value(calc_mean_accum_energy(&est_map));
        calc_drive_force_recr(&mut df_recr_map,  &subgrains_map, &est_poly_component, params.get_f64("egb"));
        //calc_drive_force_recr_cryst(&mut df_recr_cryst_map, &gr_size_map, &est_poly_component, params.get_f64("egb"));
        //calc_facet_mobility(&mut facet_mobility_map, params.get_f64("m0"), params.get_f64("Q"), params.get_f64("r"), params.get_f64("temp"));
        check_new_grain(new_grains, &df_recr_map, &mut gr_size_map, &mut subgrains_map);
        if new_grains.len()>0{
            let entity = CrystalEntity::new((gr_size_map.len()+1) as u32);

            //Заполняются все HashMap объявленные выше. 
            //Первый аргумент - сущность, второй аргумент - новый экземпляр компонента, третий аргумент - HashMap соответствующего компонента
            insert_component!(entity, RotationComponent::new(), rotation_map);
            insert_component!(entity, GradVComponent::new(), grad_v_map);
            insert_component!(entity, DComponent::new(), d_map);
            insert_component!(entity, DComponent::new(), de_map);
            insert_component!(entity, DComponent::new(), din_map);
            insert_component!(entity, SigmaComponent::new(), sigma_map);
            insert_component!(entity, SigmaRateComponent::new(), sigma_rate_map);
            insert_component!(entity, ElasticityTensorComponent::new(), elasticity_map);
            insert_component!(entity, EpsComponent::new(), eps_map);
            insert_component!(entity, BurgersVectorComponent::new(), burgers_map);
            insert_component!(entity, NormalVectorComponent::new(), normals_map);
            insert_component!(entity, BNComponent::new(), bn_map);
            insert_component!(entity, TauComponent::new(), tau_map);
            insert_component!(entity, TauComponent::new(), tau_c_map);
            insert_component!(entity, TauRateComponent::new(), tau_rate_map);
            insert_component!(entity, TauRateComponent::new(), tau_c_rate_map);
            insert_component!(entity, GammaComponent::new(), gamma_map);
            insert_component!(entity, GammaRateComponent::new(), gamma_rate_map);
            insert_component!(entity, HVectorComponent::new(), h_vector_map);
            insert_component!(entity, HMatrixComponent::new(), h_matrix_map);
            insert_component!(entity, GrainSizeComponent::new(), gr_size_map);
            insert_component!(entity, AccumEnergyComponent::new(), est_map);
            insert_component!(entity, AccumEnergyRateComponent::new(), est_rate_map);
            insert_component!(entity, StatusRecrystComponent::new(), status_map);
            insert_component!(entity, FacetMobilityComponent::new(), facet_mobility_map);
            insert_component!(entity, SubGrainsComponent::new(), subgrains_map);
            insert_component!(entity, DriveForceRecrComponent::new(), df_recr_map);
            insert_component!(entity, DriveForceRecrCrystComponent::new(), df_recr_cryst_map);
            insert_component!(entity, VelocityFacetComponent::new(), vel_facet_map);

            rotation_map.get_mut(&entity).unwrap().set_matrix(get_uniform_distribution()).unwrap();
            get_burgers_vectors(burgers_map.get_mut(&entity).unwrap());
            get_normals_vector(normals_map.get_mut(&entity).unwrap());
            get_new_bn(bn_map.get_mut(&entity).unwrap(), burgers_map.get(&entity).unwrap(), normals_map.get(&entity).unwrap());
            get_elasticity_tensor_fcc(elasticity_map.get_mut(&entity).unwrap(), params.get_f64("c11"), params.get_f64("c12"), params.get_f64("c44"), params.get_f64("koef"));
            get_tauc(tau_c_map.get_mut(&entity).unwrap(), params.get_f64("tau_c"),params.get_f64("b"), params.get_f64("k_y"), params.get_f64("gr_size"));
            get_subgrains(subgrains_map.get_mut(&entity).unwrap(), params.get_f64("r0"), params.get_i64("num_sg") as usize);
            status_map.get_mut(&entity).unwrap().set_value(true);
        }
        new_grains.clear();
    }
    //Ниже вывод финального состояния поликристалла. 
    polycrystal_sigma.set_tensor(calc_mean_sigma(&sigma_map, &rotation_map));
    polycrystal_eps.set_tensor(calc_mean_eps(&eps_map, &rotation_map));
    write_intensity_to_file(&polycrystal_eps, &polycrystal_sigma, params.get_i64("steps_num"), dt);
    print_current_sys(time.elapsed(), params.get_i64("steps_num"), params.get_i64("steps_num"), &polycrystal_eps, &polycrystal_sigma);
}
