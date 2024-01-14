//Ниже добавлены все необходимые модули
mod mmuvp;
mod consts;
mod base_fn;

//Ниже добавлены все системные функции
use std::{collections::HashMap, time::Instant};

//Ниже добавлены все используемые компоненты и системы. Вручную желательно не исправлять во избежание ошибок.
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
    }
};

use base_fn::*;



fn main() {
    //Очищение файлы в папке output. 
    clear_output_folder();

    //Задаем HashMap где будут храниться все параметры модели
    let mut params = Params::new();
    //Считываем все параметры из файла param.json
    from_file(&mut params);
    //Для удобства некоторые часто используемые параметры можно записать в отдельные переменные
    let dt = params.get_f64("dt");

    //Ниже записываются начальные условия, считывается траектория деформирования или задается деформации(гипотеза Фойгта)/напряжения(гипотеза Рейса).
    let init_grad_v = uniaxial_tension(1.0e-3);
    

    //Ниже объявляются HashMap всех компонентов. Первый аргумент - название переменной, второй - сущность, третий - компонент. 
    //Рекомендуется использовать стандартные переменные
    create_component_map!(rotation_map, CrystalEntity, RotationComponent);
    create_component_map!(rotation_rate_map, CrystalEntity, RotationRateComponent);
    create_component_map!(spin_map, CrystalEntity, SpinComponent);
    create_component_map!(grad_v_map, CrystalEntity, GradVComponent);
    create_component_map!(d_map, CrystalEntity, DComponent);
    create_component_map!(de_map, CrystalEntity, DComponent);
    create_component_map!(din_map, CrystalEntity, DComponent);
    create_component_map!(sigma_map, CrystalEntity, SigmaComponent);
    create_component_map!(sigma_rate_map, CrystalEntity, SigmaRateComponent);
    create_component_map!(elasticity_map, CrystalEntity, ElasticityTensorComponent);
    create_component_map!(w_map, CrystalEntity, WComponent);
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

    //Ниже заполняются компоненты для каждого зерна в цикле
    for i in 0..params.get_i64("grain_num"){
        //Создается новый объект 
        let entity = CrystalEntity::new(i.try_into().unwrap());

        //Заполняются все HashMap объявленные выше. 
        //Первый аргумент - сущность, второй аргумент - новый экземпляр компонента, третий аргумент - HashMap соответствующего компонента
        insert_component!(entity, RotationComponent::new(), rotation_map);
        insert_component!(entity, RotationRateComponent::new(), rotation_rate_map);
        insert_component!(entity, SpinComponent::new(), spin_map);
        insert_component!(entity, GradVComponent::new(), grad_v_map);
        insert_component!(entity, DComponent::new(), d_map);
        insert_component!(entity, DComponent::new(), de_map);
        insert_component!(entity, DComponent::new(), din_map);
        insert_component!(entity, SigmaComponent::new(), sigma_map);
        insert_component!(entity, SigmaRateComponent::new(), sigma_rate_map);
        insert_component!(entity, ElasticityTensorComponent::new(), elasticity_map);
        insert_component!(entity, WComponent::new(), w_map);
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
    }

    //Ниже инициализируются все необходимые переменные
    gen_uniform_distribution(&mut rotation_map);
    initialize_burgers_vectors(&mut burgers_map);
    initialize_normal_vectors(&mut normals_map);
    initialize_bn(&mut bn_map, &burgers_map, &normals_map);
    initialize_elasticity_tensor_fcc(&mut elasticity_map, params.get_f64("c11"), params.get_f64("c12"), params.get_f64("c44"));
    initialize_tau_c(&mut tau_c_map, params.get_f64("tau_c"));
    initialize_grad_v(&mut grad_v_map, &rotation_map, init_grad_v);
    initialize_d(&mut d_map, &grad_v_map);
    initialize_w(&mut w_map, &grad_v_map);

    //Объявление и инциализация переменных для поликристалла
    let mut polycrystal_sigma = SigmaComponent::new();
    let mut polycrystal_eps = EpsComponent::new();
    
    //Ниже можно указать вывод данных которые необходимо вывести для отсчетной конфигурации
    write_pole_figure(&rotation_map);

    //Начало временного отсчета
    let time = Instant::now();

    //Начало расчета
    for step in 0..params.get_i64("steps_num"){
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
        calc_din(&mut din_map, &gamma_rate_map, &bn_map);
        calc_de_elastic_plastic_deform(&mut de_map, &d_map, &din_map);
        calc_hooke_law(&mut sigma_rate_map, &elasticity_map, &de_map);
        calc_sigma(&mut sigma_map, &sigma_rate_map, dt);
        calc_eps(&mut eps_map, &d_map, dt);
    }
    //Ниже вывод финального состояния поликристалла. 
    polycrystal_sigma.set_tensor(calc_mean_sigma(&sigma_map, &rotation_map));
    polycrystal_eps.set_tensor(calc_mean_eps(&eps_map, &rotation_map));
    write_intensity_to_file(&polycrystal_eps, &polycrystal_sigma, params.get_i64("steps_num"), dt);
    print_current_sys(time.elapsed(), params.get_i64("steps_num"), params.get_i64("steps_num"), &polycrystal_eps, &polycrystal_sigma);
   
}
