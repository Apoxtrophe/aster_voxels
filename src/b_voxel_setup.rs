use bevy::{prelude::*, window::*, render::mesh::VertexAttributeValues};

use bevy_rapier3d::geometry::Collider;
use crate::{v_config::*, v_components::{Ground, Sun}, a_loading::TextureHandles, VoxelAssets};
use bevy::render::mesh::shape;



use crate::AppState;

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
    // initialize voxel assets
    commands.insert_resource(VoxelAssets::new(&mut meshes, &texture_handles));
    setup_lighting(&mut commands, &mut ambient_light);
    configure_window(windows.single_mut());
    // Crosshair
    spawn_ui_elements(&mut commands, &texture_handles);
    create_ground(&mut commands, &mut meshes, &mut materials, &texture_handles);
    println!("Moving onto InGame");
    next_state.set(AppState::InGame);
}

fn setup_lighting(commands: &mut Commands, ambient_light: &mut ResMut<AmbientLight>) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    }).insert(Sun);

    ambient_light.color = AMBIENT_COLOR;
    ambient_light.brightness = AMBIENT_INTENSITY;
}

fn configure_window(mut window: Mut<Window>) {
    window.title = "Logica".to_string();
    window.resolution = WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    window.present_mode = PresentMode::AutoVsync;
    window.cursor.icon = CursorIcon::Crosshair;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.window_theme = Some(WindowTheme::Dark);
    window.mode = WindowMode::BorderlessFullscreen;
    window.cursor.visible = false;
    window.decorations = true;
}

fn spawn_ui_elements(commands: &mut Commands, texture_handles: &Res<TextureHandles>) {
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
}

fn create_ground(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, texture_handles: &Res<TextureHandles>) {
    let handle_texture = texture_handles.image_handles.get(1).expect("Texture handle not found");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(handle_texture.clone()),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: WORLD_PERCIEVED_ROUGHNESS,
        metallic: WORLD_METALLIC,
        reflectance: WORLD_REFLECTANCE,
        ..Default::default()
    });

    let mut mesh: Mesh = shape::Plane { size: WORLD_SIZE as f32, subdivisions: WORLD_SIZE as u32 }.into();
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
            transform: Transform::from_translation(Vec3::new(0.5, WORLD_HEIGHT_OFFSET, 0.5)),
            ..default()
        },
        Ground,
    )).insert(Collider::cuboid(WORLD_SIZE as f32, WORLD_HEIGHT_OFFSET, WORLD_SIZE as f32));
}