use bevy::{ecs::{system::{Commands, ResMut, Res, Query}, schedule::NextState, query::With}, asset::{Assets, Handle}, render::{mesh::{Mesh, shape, VertexAttributeValues, Indices}, render_resource::PrimitiveTopology, texture}, pbr::{StandardMaterial, AmbientLight, DirectionalLightBundle, DirectionalLight, CascadeShadowConfigBuilder, PbrBundle}, window::{Window, PrimaryWindow, WindowResolution, PresentMode, CursorIcon, CursorGrabMode, WindowMode}, math::{Quat, Vec3, vec2, Vec2}, prelude::default, transform::{components::Transform, self}, ui::{node_bundles::ImageBundle, UiImage, Style, AlignSelf, PositionType, Val}, core_pipeline::core_3d::Camera3dBundle};
use bevy_rapier3d::{geometry::{Collider, ComputedColliderShape, VHACDParameters}, rapier::dynamics::RigidBody};
use noise::{Perlin, NoiseFn};
use rand::Rng;



use crate::{AppState, v_config::{SUN_ANGLE, SUN_INTENSITY, SUN_SHADOWS, SHADOW_CASCADES, SHADOW_DISTANCE, FIRST_CASCADE_BOUND, OVERLAP_PROPORTION, AMBIENT_COLOR, AMBIENT_INTENSITY, SCREEN_WIDTH, SCREEN_HEIGHT, WORLD_SIZE, SUN_LOCATION, WORLD_HEIGHT, V_TEXTURE_ATLAS_SIZE, TEXTURE_BIAS, NORMALS_MULTIPLIER, TERRIAN_ROUGHNESS, TERRAIN_HEIGHT_VARIANCE, GROUND_ROUGHNESS, GROUND_METALLIC, GROUND_RELFECTANCE}, v_components::Ground, a_loading::TextureHandles, v_graphics::VoxelAssets};

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

    //let normal = Vec3::new(0.0, 1.0, 0.0); // Normal pointing upward
    
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = Vec::new();
    let mut stat_index: u32 = 0;
    let mut rng = rand::thread_rng();

    
    let perlin = Perlin::new(rng.gen_range(0..=1000));

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



            let iness: f64 = TERRIAN_ROUGHNESS;
            let terrain_height: f32 = TERRAIN_HEIGHT_VARIANCE;

            let vy1 = perlin.get([(xi as f64 - 0.5) * iness, (zi as f64 - 0.5) * iness]);
            let vy2 = perlin.get([(xi as f64 - 0.5) * iness, (zi as f64 + 0.5) * iness]);
            let vy3 = perlin.get([(xi as f64 + 0.5) * iness, (zi as f64 - 0.5) * iness]);
            let vy4 = perlin.get([(xi as f64 + 0.5) * iness, (zi as f64 + 0.5) * iness]);

            
            let tile_vertices = [
                Vec3::new(xi - 0.5, vy1 as f32 * terrain_height, zi - 0.5), // Bottom left
                Vec3::new(xi - 0.5, vy2 as f32 * terrain_height, zi + 0.5), // Top left
                Vec3::new(xi + 0.5, vy3 as f32 * terrain_height, zi - 0.5), // Bottom right
                Vec3::new(xi + 0.5, vy4 as f32 * terrain_height, zi + 0.5), // Top right
            ];

            let offset = stat_index * 4;
            let tile_indices = [
                0 + offset, 1 + offset, 3 + offset,
                0 + offset, 3 + offset, 2 + offset,
            ];

            vertices.extend_from_slice(&tile_vertices);
            uvs.extend_from_slice(&tile_uvs);
            indices.extend_from_slice(&tile_indices);
            //normals.extend(vec![normal; 4]);
            stat_index += 1;
        }
    }   

    let mut normals = vec![Vec3::ZERO; vertices.len()];

    let normal_multiplier: f32 = NORMALS_MULTIPLIER;

    for i in (0..indices.len()).step_by(3) {
        let index1 = indices[i] as usize;
        let index2 = indices[i + 1] as usize;
        let index3 = indices[i + 2] as usize;
    
        let vertex1 = vertices[index1];
        let vertex2 = vertices[index2];
        let vertex3 = vertices[index3];
    
        let edge1 = (vertex2 - vertex1) * normal_multiplier;
        let edge2 = (vertex3 - vertex1) * normal_multiplier;
        let face_normal = edge1.cross(edge2).normalize();

        normals[index1] += face_normal;
        normals[index2] += face_normal;
        normals[index3] += face_normal;
    }

    for normal in normals.iter_mut() {
        *normal = normal.normalize();
    }

    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    combined_mesh.set_indices(Some(Indices::U32(indices)));
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(handle_texture.clone()),
        perceptual_roughness: GROUND_ROUGHNESS,
        metallic: GROUND_METALLIC,
        reflectance: GROUND_RELFECTANCE,
        ..Default::default()
    });

    let x_shape = Collider::from_bevy_mesh(&combined_mesh, &ComputedColliderShape::TriMesh).unwrap();

    commands.spawn(PbrBundle {
        mesh: meshes.add(combined_mesh),
        material: material_handle,
        transform: Transform::from_translation(Vec3::new(0.0, WORLD_HEIGHT, 0.0)),
        ..Default::default()
    }).insert(x_shape);

    println!("Moving onto InGame");
    next_state.set(AppState::InGame);

}
