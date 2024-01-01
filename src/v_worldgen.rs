use bevy::{ecs::{system::{Commands, ResMut, Res, Query}, component::Component, query::With}, asset::{Assets, AssetServer}, pbr::{StandardMaterial, AmbientLight, PbrBundle}, render::{mesh::{Mesh, shape}, color::Color}, window::{Window, PrimaryWindow}, transform::components::Transform, math::Vec3};



#[derive(Component)]
pub struct Ground;


pub fn worldgen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.1, 0.2, 0.1),
        perceptual_roughness: 0.95, // Adjust this value to make the ground appear rougher
        metallic: 0.0,
        ..Default::default()
    });
    
    let mut mesh: Mesh = shape::Plane { size: 100.0, subdivisions: 100 }.into();
    
    let mesh_handle = meshes.add(mesh);
    
    // Spawn the ground entity with the rough material
    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: ground_material,
            transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.0)),
            ..Default::default()
        },
        Ground,
    ));
}
