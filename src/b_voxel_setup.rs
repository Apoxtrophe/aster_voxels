use bevy::{ecs::{system::{Commands, ResMut, Res, Query}, schedule::NextState, query::With}, asset::{Assets, Handle}, render::{mesh::{Mesh, shape, VertexAttributeValues, Indices}, render_resource::PrimitiveTopology, texture}, pbr::{StandardMaterial, AmbientLight, DirectionalLightBundle, DirectionalLight, CascadeShadowConfigBuilder, PbrBundle}, window::{Window, PrimaryWindow, WindowResolution, PresentMode, CursorIcon, CursorGrabMode, WindowMode}, math::{Quat, Vec3, vec2, Vec2}, prelude::default, transform::{components::Transform, self}, ui::{node_bundles::ImageBundle, UiImage, Style, AlignSelf, PositionType, Val}, core_pipeline::core_3d::Camera3dBundle};
use bevy_rapier3d::geometry::Collider;
use rand::Rng;



use crate::{AppState, v_config::{SUN_ANGLE, SUN_INTENSITY, SUN_SHADOWS, SHADOW_CASCADES, SHADOW_DISTANCE, FIRST_CASCADE_BOUND, OVERLAP_PROPORTION, AMBIENT_COLOR, AMBIENT_INTENSITY, SCREEN_WIDTH, SCREEN_HEIGHT, WORLD_SIZE, SUN_LOCATION, WORLD_HEIGHT, V_TEXTURE_ATLAS_SIZE, TEXTURE_BIAS}, v_components::Ground, a_loading::TextureHandles, v_graphics::VoxelAssets};

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
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = Vec::new();
    let mut stat_index: u32 = 0;
    let mut rng = rand::thread_rng();

    for x in 0..WORLD_SIZE {
        for z in 0..WORLD_SIZE {
            let xi = x as f32;
            let zi = z as f32;  
            let mut index = rng.gen_range(0..=V_TEXTURE_ATLAS_SIZE + TEXTURE_BIAS);
            index = index.min(V_TEXTURE_ATLAS_SIZE);

            let texture_index = index as f32 / V_TEXTURE_ATLAS_SIZE as f32;
            let texture_size = 1.0 / V_TEXTURE_ATLAS_SIZE as f32;
            let (u_min, u_max) = ((texture_index - texture_size) as f32, texture_index as f32);
            let (v_min, v_max) = (0.0, 1.0);

            let tile_uvs = [
                Vec2::new(u_min, v_min),
                Vec2::new(u_min, v_max),
                Vec2::new(u_max, v_min),
                Vec2::new(u_max, v_max), 
            ];

            for &vertex_index in &vertex_indices {
                let uv = match vertex_index {
                    // Determine UV based on the corner of the tile the vertex is part of
                    0 => Vec2::new(u_min, v_min),
                    1 => Vec2::new(u_min, v_max),
                    2 => Vec2::new(u_max, v_min),
                    3 => Vec2::new(u_max, v_max),
                    _ => unreachable!(),
                };
                uvs[vertex_index] = uv;
            }

            let offset = stat_index * 4;
            let tile_indices = [
                0 + offset, 1 + offset, 3 + offset,
                0 + offset, 3 + offset, 2 + offset,
            ];

            vertices.extend_from_slice(&tile_vertices);
            uvs.extend_from_slice(&tile_uvs);
            indices.extend_from_slice(&tile_indices);
            normals.extend(vec![normal; 4]);
            stat_index += 1;
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
