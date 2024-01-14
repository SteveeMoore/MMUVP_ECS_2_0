use mmuvp::{
    entity::CrystalEntity, 
    rotation::components::*,
    rotation::systems::*, 
    elasticity::components::*,
    elasticity::systems::*
};
use nalgebra::Matrix3;
use std::collections::HashMap;

use crate::base_fn::clear_output_folder;
mod mmuvp;
mod consts;
mod base_fn;

fn main() {
    create_component_map!(rotation_map,CrystalEntity,RotationComponent);
    create_component_map!(grad_v_map,CrystalEntity,GradVComponent);
    create_component_map!(d_map, CrystalEntity, DComponent);
    create_component_map!(w_map, CrystalEntity, WComponent);

    for id in 0..1{
        let entity = CrystalEntity::new(id);
        insert_component!(entity, RotationComponent::new(), rotation_map);
        insert_component!(entity, GradVComponent::new(), grad_v_map);
        insert_component!(entity, DComponent::new(), d_map);
        insert_component!(entity, WComponent::new(), w_map);
    }
    clear_output_folder();
    gen_uniform_distribution(&mut rotation_map);
    write_pole_figure(&rotation_map);
    
    let init_grad_v = Matrix3::new(
        1e-3, 0.0, 0.0, 
        0.0, -0.5e-3, 0.0,
        0.0, 0.0, -0.5e-3
    );
    initialize_grad_v(&mut grad_v_map, &rotation_map, init_grad_v);
    initialize_d(&mut d_map, &grad_v_map);
    initialize_w(&mut w_map, &grad_v_map);
    let mut trajectory_deformation:Vec<TrajectoryDeformationComponent> = Vec::new();
    read_grad_v_from_file_with_6_comp(&mut trajectory_deformation);
    
    
}
