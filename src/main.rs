use mmuvp::{
    entity::CrystalEntity, 
    rotation::components::*,
    rotation::systems::*
};
use std::collections::HashMap;

use crate::base_fn::clear_output_folder;
mod mmuvp;
mod consts;
mod base_fn;

fn main() {
    create_component_map!(rotation_map,CrystalEntity,RotationComponent);

    for id in 0..100{
        let entity = CrystalEntity::new(id);
        insert_component!(entity.clone(), RotationComponent::new(), rotation_map);
    }
    clear_output_folder();
    gen_uniform_distribution(&mut rotation_map);
    write_pole_figure(&rotation_map);
    write_rotation_to_file(&rotation_map);
}