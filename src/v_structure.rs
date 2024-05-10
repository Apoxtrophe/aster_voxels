use crate::{
    v_components::{PositionVoxel, StateVoxel, TypeVoxel},
    v_graphics::VoxelAssets,
    v_selector::VoxelSelector,
};
use bevy::ecs::system::Query;
use bevy::ecs::system::Resource;
use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        system::{Commands, Res, ResMut},
    },
    math::IVec3,
    pbr::{PbrBundle, StandardMaterial},
    render::mesh::Mesh,
    transform::components::Transform,
};
use bevy_rapier3d::geometry::Collider;

#[derive(Resource)]
pub struct Voxel;

impl Voxel {
    pub fn new() -> Self {
        Voxel
    }

    pub fn get(
        &self,
        position: IVec3,
        get_query: Query<(&PositionVoxel, &TypeVoxel, &StateVoxel)>,
    ) -> Option<(TypeVoxel, StateVoxel)> {
        get_query
            .iter()
            .find(|(voxel_position, _, _)| voxel_position.0 == position)
            .map(|(_, voxel_type, voxel_state)| (*voxel_type, *voxel_state))
    }

    pub fn set_state(
        &mut self,
        commands: &mut Commands,
        position: IVec3,
        new_state: bool,
        mut state_query: Query<(Entity, &PositionVoxel, &mut StateVoxel)>,
    ) {
        if let Some((entity, _, mut voxel_state)) = state_query
            .iter_mut()
            .find(|(_, voxel_position, _)| voxel_position.0 == position)
        {
            *voxel_state = StateVoxel(new_state);
            commands.entity(entity).insert(StateVoxel(new_state));
        }
    }

    pub fn place(
        &mut self,
        commands: &mut Commands,
        position: IVec3,
        voxel_selector: &ResMut<VoxelSelector>,
        voxel_assets: &Res<VoxelAssets>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        state: bool,
    ) {
        let voxel_type = voxel_selector.current_voxel_type();
        let voxel_mesh_handle = voxel_assets.create_voxel_mesh(voxel_type, &mut meshes);
        let atlas_material = voxel_assets.atlas_material(&mut materials);

        commands
            .spawn((
                PbrBundle {
                    mesh: voxel_mesh_handle,
                    material: atlas_material,
                    transform: Transform::from_translation(position.as_vec3()),
                    ..Default::default()
                },
                PositionVoxel(position),
                voxel_type,
                StateVoxel(state),
                Collider::cuboid(0.5, 0.5, 0.5),
            ));
    }

    pub fn lean_place(
        &mut self,
        commands: &mut Commands,
        position: IVec3,
        voxel_type: TypeVoxel,
        state: bool,
        voxel_assets: &Res<VoxelAssets>,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let voxel_mesh_handle = voxel_assets.create_voxel_mesh(voxel_type, &mut meshes);
        let atlas_material = voxel_assets.atlas_material(materials);

        commands.spawn((
            PbrBundle {
                mesh: voxel_mesh_handle,
                material: atlas_material,
                transform: Transform::from_translation(position.as_vec3()),
                ..Default::default()
            },
            PositionVoxel(position),
            voxel_type,
            StateVoxel(state),
            Collider::cuboid(0.5, 0.5, 0.5),
        ));
    }

    pub fn remove(
        &mut self,
        commands: &mut Commands,
        position: IVec3,
        query: Query<(Entity, &PositionVoxel)>,
    ) {
        for (entity, voxel_position) in query.iter() {
            if voxel_position.0 == position {
                commands.entity(entity).despawn();
            }
        }
    }
}
