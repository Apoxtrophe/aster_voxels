use bevy::{ecs::{system::{Commands, ResMut, Res, Query}, schedule::NextState, query::With, world::World}, asset::Assets, render::{mesh::{Mesh, shape, VertexAttributeValues, Indices}, texture, render_resource::PrimitiveTopology}, pbr::{StandardMaterial, AmbientLight, DirectionalLightBundle, DirectionalLight, CascadeShadowConfigBuilder, PbrBundle}, window::{Window, PrimaryWindow, WindowResolution, PresentMode, CursorIcon, CursorGrabMode, WindowMode}, math::{Quat, Vec3}, prelude::default, transform::components::Transform, ui::{node_bundles::ImageBundle, UiImage, Style, AlignSelf, PositionType, Val}, core_pipeline::core_3d::Camera3dBundle};
use bevy_atmosphere::plugin::AtmosphereCamera;
use rand::Rng;



use crate::{AppState, v_config::{SUN_ANGLE, SUN_INTENSITY, SUN_SHADOWS, SHADOW_CASCADES, SHADOW_DISTANCE, FIRST_CASCADE_BOUND, OVERLAP_PROPORTION, AMBIENT_COLOR, AMBIENT_INTENSITY, SCREEN_WIDTH, SCREEN_HEIGHT, WORLD_SIZE}, v_components::{CameraRotation, Ground}, a_loading::TextureHandles, v_graphics::VoxelAssets};

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
            translation: Vec3::new(10.0, 2.0, 0.0),
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

    // Create the player
    commands
            .spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            })
            .insert(CameraRotation {
                pitch: 0.0,
                yaw: 0.0,
            })
            .insert(AtmosphereCamera::default());

    // Create the ground
    
    let handle_texture = texture_handles.image_handles.get(1).expect("Texture handle not found");


    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(handle_texture.clone()),
        ..Default::default()
    });


    let mut mesh : Mesh = shape::Plane { size: WORLD_SIZE as f32, subdivisions: WORLD_SIZE as u32}.into(); 
    let atlas_width = 8; // Adjust as per your texture atlas
    let subdivision_size = 1.0 / WORLD_SIZE as f32;
    let texture_size = 1.0 / atlas_width as f32;
    let mut rng = rand::thread_rng();

    if let VertexAttributeValues::Float32x2(uvs) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap() {
        for segment_x in 0..WORLD_SIZE {
            for segment_y in 0..WORLD_SIZE {
                // Randomly select a texture for this segment
                let texture_index = rng.gen_range(0..atlas_width);
                let left = texture_index as f32 * texture_size;
                let right = left + texture_size;

                // Assuming top and bottom are constant since the atlas is 1D
                let top = 1.0;
                let bottom = 0.0;

                // UVs for this segment (2 triangles, 6 vertices)
                let segment_uvs = [
                    [left, bottom], [right, bottom], [right, top], // First triangle
                    [right, top], [left, top], [left, bottom]     // Second triangle
                ];

                // Calculate the index of the first vertex of this segment
                let vertex_index = (segment_x + segment_y * WORLD_SIZE) * 6;

                // Assign the calculated UVs to the vertices of this segment
                for i in 0..6 {
                    uvs[vertex_index as usize + i] = segment_uvs[i];
                }
            }
        }
    }

    let mesh_handle = meshes.add(mesh);


    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: material_handle,
            ..default()
        },
        Ground,
    ));



    /* 
    let handle_texture = texture_handles.image_handles.get(1).expect("Texture handle not found");


    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(handle_texture.clone()),
        ..Default::default()
    });


    let mut mesh : Mesh = shape::Plane { size: WORLD_SIZE as f32, subdivisions: WORLD_SIZE as u32}.into(); 
    let uvs = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
    match uvs {
        VertexAttributeValues::Float32x2(values) => {
            for uv in values.iter_mut() {
                uv[0] *= WORLD_SIZE as f32;
                uv[1] *= WORLD_SIZE as f32; 
            }
        },
        _ => (),
    };

    let mesh_handle = meshes.add(mesh);


    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: material_handle,
            ..default()
        },
        Ground,
    ));
    */

    println!("Moving onto InGame");
    next_state.set(AppState::InGame);
}


fn calculate_uv_coordinates(texture_index: u32, atlas_width: u32) -> Vec<[f32; 2]> {
    let texture_size = 1.0 / atlas_width as f32;

    let left = texture_index as f32 * texture_size;
    let right = left + texture_size;
    let top = 0.0;
    let bottom = 1.0;

    vec![
        [left, top],
        [right, top],
        [right, bottom],
        [left, bottom],
    ]
}
