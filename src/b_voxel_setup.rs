use bevy::{prelude::*, render::mesh::VertexAttributeValues, window::*};
use crate::AppState;
use crate::{
    a_loading::TextureHandles,
    v_components::{Ground, Sun},
    v_config::*,
    v_graphics::VoxelAssets,
};
use bevy::render::mesh::shape;
use bevy_rapier3d::geometry::Collider;

pub fn voxel_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
    mut next_state: ResMut<NextState<AppState>>,
    texture_handles: Res<TextureHandles>,
) {
    println!("Beginning GameSetup");
    commands.insert_resource(VoxelAssets::new(&mut meshes, &texture_handles));
    setup_lighting(&mut commands, &mut ambient_light);
    spawn_ui_elements(&mut commands, &texture_handles);
    create_ground(&mut commands, &mut meshes, &mut materials, &texture_handles);

    println!("Moving onto InGame");
    next_state.set(AppState::InGame);
}

fn setup_lighting(commands: &mut Commands, ambient_light: &mut ResMut<AmbientLight>) {
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Sun);

    ambient_light.color = AMBIENT_COLOR;
    ambient_light.brightness = AMBIENT_INTENSITY;
}

fn spawn_ui_elements(commands: &mut Commands, texture_handles: &Res<TextureHandles>) {
    let crosshair_handle = texture_handles
        .image_handles
        .get(2)
        .expect("Texture handle not found");
    commands.spawn(ImageBundle {
        image: UiImage {
            texture: (crosshair_handle.clone()),
            flip_x: (false),
            flip_y: (false),
        },
        style: Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            left: Val::Px((SCREEN_WIDTH / 2.0) - 32.0),
            top: Val::Px((SCREEN_HEIGHT / 2.0) - 32.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn create_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    texture_handles: &Res<TextureHandles>,
) {
    if let Some(handle_texture) = texture_handles.image_handles.get(1) {
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(handle_texture.clone()),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: WORLD_PERCIEVED_ROUGHNESS,
            metallic: WORLD_METALLIC,
            reflectance: WORLD_REFLECTANCE,
            ..Default::default()
        });

        let mut mesh: Mesh = shape::Plane {
            size: WORLD_SIZE as f32,
            subdivisions: WORLD_SIZE as u32,
        }
        .into();
        if let VertexAttributeValues::Float32x2(values) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap()
        {
            for uv in values.iter_mut() {
                uv[0] *= WORLD_SIZE as f32;
                uv[1] *= WORLD_SIZE as f32;
            }
        }
        let mesh_handle = meshes.add(mesh);

        commands
            .spawn((
                PbrBundle {
                    mesh: mesh_handle,
                    material: material_handle,
                    transform: Transform::from_translation(Vec3::new(
                        0.5,
                        WORLD_HEIGHT_OFFSET,
                        0.5,
                    )),
                    ..default()
                },
                Ground,
            ))
            .insert(Collider::cuboid(
                WORLD_SIZE as f32,
                WORLD_HEIGHT_OFFSET,
                WORLD_SIZE as f32,
            ));
    }
}
