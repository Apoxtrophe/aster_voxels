use std::f32::consts::TAU;

use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    window::CursorGrabMode, input::mouse::MouseWheel,
};
use bevy_atmosphere::{plugin::AtmosphereCamera, system_param::Atmosphere};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

use crate::{v_selector::{vox_scroll_selection, VoxelSelector}, v_lib::VoxelInfo, v_graphics::VoxelAssets, v_components::{TypeVoxel, PositionVoxel, StateVoxel}, v_structure::Voxel, v_config::{FIELD_OF_VIEW, PITCH_SPEED, YAW_SPEED, AIR_ACCELERATION, CAMERA_HEIGHT, CAMERA_RADIUS, PLAYER_HEIGHT}};

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub fn player_setup(
    mut commands: Commands,
    mut window: Query<&mut Window>,
    assets: Res<AssetServer>,
) {

    
    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    let logical_entity = commands
        .spawn((
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(Transform::from_translation(SPAWN_POINT)),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / PITCH_SPEED,
                yaw: TAU * 5.0 / YAW_SPEED,
                ..default()
            },
            FpsController {
                air_acceleration: AIR_ACCELERATION,
                upright_height: PLAYER_HEIGHT,
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: CAMERA_HEIGHT,
            radius_scale: CAMERA_RADIUS,
        })
        .id();

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / FIELD_OF_VIEW,
                ..default()
            }),
            ..default()
        },
        AtmosphereCamera::default(),
        RenderPlayer { logical_entity },
    ));

    commands.spawn(TextBundle::from_section(
        "",
        TextStyle {
            font: assets.load("fira_mono.ttf"),
            font_size: 24.0,
            color: Color::BLACK,
        },
    ).with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(5.0),
        left: Val::Px(5.0),
        ..default()
    }));
}

pub fn respawn(
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}

pub fn manage_cursor(
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,

    mouse_wheel_events: EventReader<MouseWheel>,

    mut voxel_selector: ResMut<VoxelSelector>,
) {

    vox_scroll_selection(mouse_wheel_events, &mut voxel_selector);

    let mut window = window_query.single_mut();
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}

pub fn display_text(
    mut controller_query: Query<(&Transform, &Velocity)>,
    mut text_query: Query<&mut Text>,
) {
    for (transform, velocity) in &mut controller_query {
        for mut text in &mut text_query {
            text.sections[0].value = format!(
                "vel: {:.2}, {:.2}, {:.2}\npos: {:.2}, {:.2}, {:.2}\nspd: {:.2}",
                velocity.linvel.x, velocity.linvel.y, velocity.linvel.z,
                transform.translation.x, transform.translation.y, transform.translation.z,
                velocity.linvel.xz().length()
            );
        }
    }
}

pub fn voxel_interaction_system(
    mouse_input: Res<Input<MouseButton>>,
    voxel_assets: Res<VoxelAssets>,
    voxel_selector: ResMut<VoxelSelector>,
    mut commands: Commands,
    mut voxel: ResMut<Voxel>,
    voxel_info: Res<VoxelInfo>,
    keyboard_input: Res<Input<KeyCode>>,
    remove_query: Query<(Entity, &PositionVoxel)>,
    state_query: Query<(Entity, &PositionVoxel, &mut StateVoxel)>,
    materials: ResMut<Assets<StandardMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    //Placing, removing, and altering state on mouse click
    

    if voxel_info.in_range {
        if mouse_input.just_pressed(MouseButton::Left) {
            if keyboard_input.pressed(KeyCode::ControlLeft) {
                if let Some(state) = voxel_info.is_on {
                    if let Some(voxel_type) = voxel_info.voxel_type {
                        if voxel_type == TypeVoxel::Switch {
                            voxel.set_state(
                                &mut commands,
                                voxel_info.position,
                                !state,
                                state_query,
                            );
                        }
                    }
                }
            } else {
                voxel.place(
                    &mut commands,
                    voxel_info.adjacent,
                    &voxel_selector,
                    &voxel_assets,
                    materials,
                    meshes,
                )
            }
        } else if mouse_input.just_pressed(MouseButton::Right) {
            voxel.remove(&mut commands, voxel_info.position, remove_query);
        }
    }
}
