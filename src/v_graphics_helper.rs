use crate::v_config::VOXEL_ATLAS_SIZE;

pub fn calculate_uv_coordinates(texture_index: u32) -> Vec<[f32; 2]> {
    let atlas_width = VOXEL_ATLAS_SIZE as f32;
    let texture_size = 1.0 / atlas_width;

    let left = texture_index as f32 * texture_size;
    let right = left + texture_size;
    let top = 0.0;
    let bottom = 1.0;

    let alternate_left = (texture_index + 9) as f32 * texture_size;
    let alternate_right = alternate_left + texture_size;

    let mut uv_coordinates = Vec::with_capacity(24);
    for _ in 0..3 {
        uv_coordinates.extend_from_slice(&[
            [left, top],
            [right, top],
            [right, bottom],
            [left, bottom],
        ]);
        uv_coordinates.extend_from_slice(&[
            [alternate_left, top],
            [alternate_right, top],
            [alternate_right, bottom],
            [alternate_left, bottom],
        ]);
    }

    uv_coordinates
}

pub fn calculate_positions() -> Vec<[f32; 3]> {
    vec![
        // top (facing towards +y)
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
        // bottom (-y)
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
        // right (+x)
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        // left (-x)
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        // back (+z)
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],
        // forward (-z)
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
    ]
}

pub fn calculate_normals() -> Vec<[f32; 3]> {
    vec![
        // Normals for the top side (towards +y)
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        // Normals for the bottom side (towards -y)
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        // Normals for the right side (towards +x)
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        // Normals for the left side (towards -x)
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        // Normals for the back side (towards +z)
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        // Normals for the forward side (towards -z)
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
    ]
}

pub fn calculate_indices() -> Vec<u32> {
    vec![
        0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
        4, 5, 7, 5, 6, 7, // bottom (-y)
        8, 11, 9, 9, 11, 10, // right (+x)
        12, 13, 15, 13, 14, 15, // left (-x)
        16, 19, 17, 17, 19, 18, // back (+z)
        20, 21, 23, 21, 22, 23, // forward (-z)
    ]
}
