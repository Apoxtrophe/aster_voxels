use bevy::{ecs::{system::{Commands, ResMut, Res, Query}, schedule::NextState, query::With}, asset::{Assets, AssetServer, Handle}, render::{mesh::{Mesh, shape}, texture::{Image, ImageSampler}, render_resource::{SamplerDescriptor, AddressMode}}, pbr::{StandardMaterial, AmbientLight, DirectionalLightBundle, DirectionalLight, CascadeShadowConfigBuilder, PbrBundle}, window::{Window, PrimaryWindow, WindowResolution, PresentMode, CursorIcon, CursorGrabMode, WindowMode}, math::{Quat, Vec3}, prelude::default, transform::components::Transform, ui::{node_bundles::ImageBundle, UiImage, Style, AlignSelf, PositionType, Val}, core_pipeline::core_3d::Camera3dBundle};
use bevy_atmosphere::plugin::AtmosphereCamera;



use crate::{AppState, v_config::{SUN_ANGLE, SUN_INTENSITY, SUN_SHADOWS, SHADOW_CASCADES, SHADOW_DISTANCE, FIRST_CASCADE_BOUND, OVERLAP_PROPORTION, AMBIENT_COLOR, AMBIENT_INTENSITY, SCREEN_WIDTH, SCREEN_HEIGHT, WORLD_SIZE}, v_components::{CameraRotation, Ground}, a_loading::Texture_Handles, v_graphics::VoxelAssets};

pub fn voxel_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut ambient_light: ResMut<AmbientLight>,
    mut next_state: ResMut<NextState<AppState>>,
    mut images: ResMut<Assets<Image>>,
    texture_handles: Res<Texture_Handles>, 
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


    
    // Ground
    let ground_handle = texture_handles.image_handles.get(1).expect("Texture handle not found");

    if let Some(texture) = images.get_mut(ground_handle) {
        println!("Texture loaded");
        texture.sampler = ImageSampler::Descriptor(SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            ..default()
        }
        .into());
    }

    let ground_material = materials.add(StandardMaterial {
        base_color_texture: Some(ground_handle.clone()),
        ..Default::default()
    });

    let mesh: Mesh = shape::Plane {
        size: WORLD_SIZE as f32,
        subdivisions: WORLD_SIZE as u32,
    }
    .into();

    let mesh_handle = meshes.add(mesh);

    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: ground_material,
            transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.0)),
            ..Default::default()
        },
        Ground,
    ));

    println!("Moving onto InGame");
    next_state.set(AppState::InGame);
}
