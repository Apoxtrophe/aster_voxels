use std::f32::consts::TAU;

use bevy::{
    input::mouse::MouseWheel, prelude::*, render::color, window::CursorGrabMode
};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

use crate::{v_components::{TypeVoxel, PositionVoxel, StateVoxel}, v_config::{PLAYER_FOV, PLAYER_PITCH_SPEED, PLAYER_YAW_SPEED, PLAYER_AIR_ACCELERATION, PLAYER_CAMERA_HEIGHT, PLAYER_CAMERA_RADIUS, PLAYER_HEIGHT}, v_graphics::VoxelAssets, v_hotbar::FadeTimer, v_lib::VoxelInfo, v_selector::VoxelSelector, v_structure::Voxel};

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub fn player_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

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
                pitch: (-TAU / PLAYER_PITCH_SPEED) * 0.0,
                yaw: (TAU * 5.0 / PLAYER_YAW_SPEED) * 0.0,
                ..default()
            },
            FpsController {
                move_mode: MoveMode::Ground,
                radius: 0.0,
                gravity: 20.0,

                air_acceleration: 10.0,
                fly_speed: 10.0,
                fast_fly_speed: 20.0,
                fly_friction: 0.04, 

                walk_speed: 10.0,
                run_speed: 20.0,
                jump_speed: 8.0,
                acceleration: 10.0,
                friction: 8.0,
                crouched_speed: 5.0, 
                crouch_speed: 8.0,
                uncrouch_speed: 8.0,
                upright_height: 2.0,
                crouch_height: 1.5,

                stop_speed: 1.0,
                sensitivity: 0.0005,
                enable_input: true, 
                step_offset: 1.0,

                key_forward: KeyCode::W,
                key_back: KeyCode::S,
                key_left: KeyCode::A,
                key_right: KeyCode::D,
                key_up: KeyCode::Space,
                key_down: KeyCode::AltLeft,
                key_sprint: KeyCode::ShiftLeft,
                key_jump: KeyCode::Space,
                key_crouch: KeyCode::AltLeft,
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: PLAYER_CAMERA_HEIGHT,
            radius_scale: PLAYER_CAMERA_RADIUS,
        })
        .id();

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / PLAYER_FOV,
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
            font: assets.load("Fonts/Retro Gaming.ttf"),
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

    mut wheel: EventReader<MouseWheel>,

    mut voxel_selector: ResMut<VoxelSelector>,

    mut query: Query<&mut BorderColor>,

    mut countdown_timer: ResMut<FadeTimer>,
) {
    // Update selected voxel && Hotbar selection && Fading text timer
    for event in wheel.read() {
        if event.y < 0.0 {
            voxel_selector.next();

            countdown_timer.active = true;
            countdown_timer.timer.reset();
            
        } else if event.y > 0.0 {
            voxel_selector.previous();

            countdown_timer.active = true;
            countdown_timer.timer.reset();
        }
        for (i, mut border_color) in query.iter_mut().enumerate() {
            if i == voxel_selector.current_index {
                border_color.0 = color::Color::LIME_GREEN.into();
            } else {
                border_color.0 = color::Color::DARK_GRAY.into();
            }
        }
    }

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
                    false,
                )
            }
        } else if mouse_input.just_pressed(MouseButton::Right) {
            voxel.remove(&mut commands, voxel_info.position, remove_query);
        }
    }
}
