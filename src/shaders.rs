use bevy::prelude::*;

pub fn init_unlit_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    color: Color,
) -> Handle<StandardMaterial> {
    let unlit_material = StandardMaterial {
        base_color: color,
        unlit: true,
        ..Default::default()
    };
    materials.add(unlit_material)
}
