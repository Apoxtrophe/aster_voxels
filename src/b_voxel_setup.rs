use bevy::{ecs::{system::{Commands, ResMut, Res, Query}, schedule::NextState, query::With}, asset::{Assets, Handle}, render::{mesh::{Mesh, shape, VertexAttributeValues, Indices}, render_resource::PrimitiveTopology, texture}, pbr::{StandardMaterial, AmbientLight, DirectionalLightBundle, DirectionalLight, CascadeShadowConfigBuilder, PbrBundle}, window::{Window, PrimaryWindow, WindowResolution, PresentMode, CursorIcon, CursorGrabMode, WindowMode}, math::{Quat, Vec3, vec2, Vec2}, prelude::default, transform::{components::Transform, self}, ui::{node_bundles::ImageBundle, UiImage, Style, AlignSelf, PositionType, Val}, core_pipeline::core_3d::Camera3dBundle};
use bevy_rapier3d::geometry::Collider;
use rand::Rng;



use crate::{AppState, v_config::{SUN_ANGLE, SUN_INTENSITY, SUN_SHADOWS, SHADOW_CASCADES, SHADOW_DISTANCE, FIRST_CASCADE_BOUND, OVERLAP_PROPORTION, AMBIENT_COLOR, AMBIENT_INTENSITY, SCREEN_WIDTH, SCREEN_HEIGHT, WORLD_SIZE, SUN_LOCATION, WORLD_HEIGHT, V_TEXTURE_ATLAS_SIZE}, v_components::Ground, a_loading::TextureHandles, v_graphics::VoxelAssets};

pub fn voxel_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut ambient_light: ResMut<AmbientLight>,
    mut next_state: ResMut<NextState<AppState>>,
    texture_handles: Res<TextureHandles>, 
) {

    println!("Beginning GameSetup");

    // Really hate that this is initialized here and not a_loading
    commands.insert_resource(VoxelAssets::new(
        &mut meshes,
        &texture_handles,
    ));


    //SUN
    let sun_radians = SUN_ANGLE.to_radians();
    let direction = Quat::from_rotation_x(-sun_radians);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: SUN_INTENSITY,
            shadows_enabled: SUN_SHADOWS,
            ..default()
        },
        transform: Transform {
            translation: SUN_LOCATION.into(),
            rotation: direction,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: SHADOW_CASCADES,
            maximum_distance: SHADOW_DISTANCE,
            first_cascade_far_bound: FIRST_CASCADE_BOUND,
            overlap_proportion: OVERLAP_PROPORTION,
            ..default()
        }
        .into(),
        ..default()
    });
    // Ambient lighting
    ambient_light.color = AMBIENT_COLOR;
    ambient_light.brightness = AMBIENT_INTENSITY; // Adjust the brightness as needed

    // Window settings
    let mut window = windows.single_mut();
    window.title = "Logica".to_string();
    window.resolution = WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    window.present_mode = PresentMode::AutoVsync;
    window.cursor.icon = CursorIcon::Crosshair;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.mode = WindowMode::Windowed;
    window.cursor.visible = false;
    window.decorations = true;
    window.window_theme = Some(bevy::window::WindowTheme::Dark);

    // Crosshair
    let crosshair_handle = texture_handles.image_handles.get(2).expect("Texture handle not found");
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

    let handle_texture = texture_handles.image_handles.get(1).expect("Texture handle not found");

    let mut combined_mesh = Mesh::new(PrimitiveTopology::TriangleList);


    let normal = Vec3::new(0.0, 1.0, 0.0); // Normal pointing upward
    let mut normals: Vec<Vec3> = Vec::new();
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut uvs: Vec<Vec2> = Vec::new();
    let mut stat_index: u32 = 0;
    let mut rng = rand::thread_rng();
    for x in 0..WORLD_SIZE {
        for z in 0..WORLD_SIZE {

            
            let offset = stat_index * 4;
            let xi = x as f32;
            let zi = z as f32;  


            let index = rng.gen_range(0..V_TEXTURE_ATLAS_SIZE);

            let texture_index = index as f32 / V_TEXTURE_ATLAS_SIZE as f32;
            let texture_size = 1.0 / V_TEXTURE_ATLAS_SIZE as f32;
            let tile_uvs = [
                Vec2::new(texture_index - texture_size, 0.0),  // UV for the first vertex
                Vec2::new(texture_index - texture_size, 1.0),  // UV for the second vertex
                Vec2::new(texture_index, 0.0),
                Vec2::new(texture_index, 1.0), 
            ];

            let tile_vertices = [
                Vec3::new(xi - 0.5, 0.0, zi - 0.5), // Bottom left
                Vec3::new(xi - 0.5, 0.0, zi + 0.5), // Top left
                Vec3::new(xi + 0.5, 0.0, zi - 0.5), // Bottom right
                Vec3::new(xi + 0.5, 0.0, zi + 0.5), // Top right
            ];

            let tile_indices = [
                0 + offset, 1 + offset, 3 + offset, // indices for the first triangle
                0 + offset, 3 + offset, 2 + offset, // indices for the second triangle
            ];


            for vertex in &tile_vertices {
                vertices.push(*vertex);
            }

            for uv in &tile_uvs {
                uvs.push(*uv);
            }

            indices.extend_from_slice(&tile_indices);
            stat_index += 1;

            
            for _ in 0..4 { // As each tile has 4 vertices
                normals.push(normal);
            }
        }
    }

    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    combined_mesh.set_indices(Some(Indices::U32(indices)));
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(handle_texture.clone()),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(combined_mesh),
        material: material_handle,
        transform: Transform::from_translation(Vec3::new(0.0, WORLD_HEIGHT, 0.0)),
        ..Default::default()
    });

    commands.spawn(Collider::cuboid(1000.0, 0.5, 1000.0));


    println!("Moving onto InGame");
    next_state.set(AppState::InGame);

}
