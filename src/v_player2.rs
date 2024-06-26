use std::{cmp::Ordering, f32::consts::TAU, time::Duration};
use bevy::{input::mouse::MouseWheel, prelude::*, render::color, window::CursorGrabMode};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::*;
use crate::{
    v_components::{MainCamera, PositionVoxel, StateVoxel, TypeVoxel},
    v_config::{
        PLAYER_CAMERA_HEIGHT, PLAYER_CAMERA_RADIUS, PLAYER_FOV, PLAYER_PITCH_SPEED,
        PLAYER_YAW_SPEED,
    },
    v_graphics::VoxelAssets,
    v_hotbar::FadeTimer,
    v_lib::VoxelInfo,
    v_selector::VoxelSelector,
    v_structure::Voxel,
    v_plugins::SpeedBar,
};
const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub fn player_setup(mut commands: Commands, assets: Res<AssetServer>) {
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
            Ccd::enabled(),
            TransformBundle::from_transform(Transform::from_translation(SPAWN_POINT)),
            LogicalPlayer,
            FpsControllerInput {
                pitch: (-TAU / PLAYER_PITCH_SPEED),
                yaw: (TAU * 5.0 / PLAYER_YAW_SPEED),
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
                key_forward: KeyCode::KeyW,
                key_back: KeyCode::KeyS,
                key_left: KeyCode::KeyA,
                key_right: KeyCode::KeyD,
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
        MainCamera,
    ));

    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font: assets.load("Fonts/Retro Gaming.ttf"),
                font_size: 24.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    );
}

pub fn respawn(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y <= -50.0 {
            velocity.linvel = Vec3::ZERO;
            transform.translation = SPAWN_POINT;
        }
    }
}

pub fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
    mut wheel: EventReader<MouseWheel>,
    mut voxel_selector: ResMut<VoxelSelector>,
    mut query: Query<&mut BorderColor>,
    mut countdown_timer: ResMut<FadeTimer>,
) {
    for event in wheel.read() {
        match event.y.partial_cmp(&0.0) {
            Some(Ordering::Less) => voxel_selector.next(),
            Some(Ordering::Greater) => voxel_selector.previous(),
            _ => (),
        }
        countdown_timer.timer.reset();

        for (i, mut border_color) in query.iter_mut().enumerate() {
            border_color.0 = if i == voxel_selector.current_index {
                Color::LIME_GREEN.into()
            } else {
                Color::DARK_GRAY.into()
            };
        }
    }

    let mut window = window_query.single_mut();
    let grab_mode = match btn.just_pressed(MouseButton::Left) {
        true => CursorGrabMode::Locked,
        false => match key.just_pressed(KeyCode::Escape) {
            true => CursorGrabMode::None,
            false => window.cursor.grab_mode,
        },
    };

    window.cursor.grab_mode = grab_mode;
    window.cursor.visible = matches!(grab_mode, CursorGrabMode::None);

    for mut controller in &mut controller_query {
        controller.enable_input = matches!(grab_mode, CursorGrabMode::Locked);
    }
}

pub fn voxel_interaction_system(
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    voxel_assets: Res<VoxelAssets>,
    voxel_selector: ResMut<VoxelSelector>,
    mut commands: Commands,
    mut voxel: ResMut<Voxel>,
    voxel_info: Res<VoxelInfo>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    remove_query: Query<(Entity, &PositionVoxel)>,
    state_query: Query<(Entity, &PositionVoxel, &mut StateVoxel)>,
    materials: ResMut<Assets<StandardMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    mut place_timer: Local<Timer>,
    mut remove_timer: Local<Timer>,
    mut speed_bar: ResMut<SpeedBar>,
) {
    let place_delay = Duration::from_millis(200);
    let remove_delay = Duration::from_millis(100);

    speed_bar.speed_index = match keyboard_input.just_pressed(KeyCode::BracketRight) {
        true => speed_bar.speed_index.clamp(1, 4) + 1,
        false => match keyboard_input.just_pressed(KeyCode::BracketLeft) {
            true => speed_bar.speed_index.clamp(2, 5) - 1,
            false => speed_bar.speed_index,
        },
    };

    if voxel_info.in_range {
        if (mouse_input.just_pressed(MouseButton::Left)
            || (mouse_input.pressed(MouseButton::Left) && place_timer.tick(time.delta()).finished()))
            && !keyboard_input.pressed(KeyCode::ControlLeft)
        {
            voxel.place(
                &mut commands,
                voxel_info.adjacent,
                &voxel_selector,
                &voxel_assets,
                materials,
                meshes,
                false,
            );
            place_timer.reset();
            place_timer.set_duration(place_delay);
        }

        if let (Some(state), Some(TypeVoxel::Switch)) = (voxel_info.is_on, voxel_info.voxel_type) {
            if mouse_input.just_pressed(MouseButton::Left) && keyboard_input.pressed(KeyCode::ControlLeft) {
                voxel.set_state(&mut commands, voxel_info.position, !state, state_query);
            }
        }

        if mouse_input.just_pressed(MouseButton::Right)
            || (mouse_input.pressed(MouseButton::Right) && remove_timer.tick(time.delta()).finished())
        {
            voxel.remove(&mut commands, voxel_info.position, remove_query);
            remove_timer.reset();
            remove_timer.set_duration(remove_delay);
        }
    }
}