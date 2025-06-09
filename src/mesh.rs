use crate::vx;
use crate::vx::Vx;
use cgmath::{Array, Quaternion, Vector3, Zero}; // Assuming you're using `glam` for vectors/quaternions
use std::f32::consts::PI;

pub fn generate_unit_sphere_mesh(lat_segments: u32, lon_segments: u32) -> (Vec<Vx>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for lat in 0..=lat_segments {
        let theta = lat as f32 * PI / lat_segments as f32;

        for lon in 0..=lon_segments {
            let phi = lon as f32 * 2.0 * PI / lon_segments as f32;

            let x = theta.sin() * phi.cos();
            let y = theta.cos();
            let z = theta.sin() * phi.sin();

            let r = (x + 1.0) * 0.5;
            let g = (y + 1.0) * 0.5;
            let b = (z + 1.0) * 0.5;

            vertices.push(vx![x, y, z => r, g, b]);
        }
    }

    for lat in 0..lat_segments {
        for lon in 0..lon_segments {
            let current = lat * (lon_segments + 1) + lon;
            let next = current + lon_segments + 1;

            if lat == 0 {
                // Top cap - only one triangle per longitude segment
                indices.push(current as u16);
                indices.push((next + 1) as u16);
                indices.push(next as u16);
            } else if lat == lat_segments - 1 {
                // Bottom cap - only one triangle per longitude segment
                indices.push(current as u16);
                indices.push((current + 1) as u16);
                indices.push(next as u16);
            } else {
                // Middle sections - two triangles per quad
                indices.push(current as u16);
                indices.push((current + 1) as u16);
                indices.push(next as u16);

                indices.push((current + 1) as u16);
                indices.push((next + 1) as u16);
                indices.push(next as u16);
            }
        }
    }

    (vertices, indices)
}

pub fn generate_icosphere_mesh(subdivisions: u32) -> (Vec<Vx>, Vec<u16>) {
    // Start with icosahedron vertices (12 vertices, 20 faces)
    let t = (1.0 + 5.0_f32.sqrt()) / 2.0; // Golden ratio

    let mut vertices = vec![
        vx![-1.0,  t, 0.0 => 1.0, 1.0, 1.0],
        vx![ 1.0,  t, 0.0 => 1.0, 1.0, 1.0],
        vx![-1.0, -t, 0.0 => 1.0, 1.0, 1.0],
        vx![ 1.0, -t, 0.0 => 1.0, 1.0, 1.0],
        vx![0.0, -1.0,  t => 1.0, 1.0, 1.0],
        vx![0.0,  1.0,  t => 1.0, 1.0, 1.0],
        vx![0.0, -1.0, -t => 1.0, 1.0, 1.0],
        vx![0.0,  1.0, -t => 1.0, 1.0, 1.0],
        vx![ t, 0.0, -1.0 => 1.0, 1.0, 1.0],
        vx![ t, 0.0,  1.0 => 1.0, 1.0, 1.0],
        vx![-t, 0.0, -1.0 => 1.0, 1.0, 1.0],
        vx![-t, 0.0,  1.0 => 1.0, 1.0, 1.0],
    ];

    // Normalize to unit sphere
    for vertex in &mut vertices {
        let len = (vertex.pos[0] * vertex.pos[0]
            + vertex.pos[1] * vertex.pos[1]
            + vertex.pos[2] * vertex.pos[2])
            .sqrt();
        vertex.pos[0] /= len;
        vertex.pos[1] /= len;
        vertex.pos[2] /= len;
    }

    // Initial icosahedron faces
    let mut indices = vec![
        // 5 faces around point 0
        0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11, // 5 adjacent faces
        1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6, 7, 1, 8, // 5 faces around point 3
        3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9, // 5 adjacent faces
        4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6, 7, 9, 8, 1,
    ];

    // Subdivide triangles
    for _ in 0..subdivisions {
        let mut new_indices = Vec::new();
        let mut edge_map: std::collections::HashMap<(u16, u16), u16> =
            std::collections::HashMap::new();

        for triangle in indices.chunks(3) {
            let v1 = triangle[0];
            let v2 = triangle[1];
            let v3 = triangle[2];

            // Get or create midpoint vertices
            let a = get_or_create_midpoint(&mut vertices, &mut edge_map, v1, v2);
            let b = get_or_create_midpoint(&mut vertices, &mut edge_map, v2, v3);
            let c = get_or_create_midpoint(&mut vertices, &mut edge_map, v3, v1);

            // Create 4 new triangles
            new_indices.extend_from_slice(&[v1, a, c]);
            new_indices.extend_from_slice(&[v2, b, a]);
            new_indices.extend_from_slice(&[v3, c, b]);
            new_indices.extend_from_slice(&[a, b, c]);
        }

        indices = new_indices;
    }

    (vertices, indices)
}

fn get_or_create_midpoint(
    vertices: &mut Vec<Vx>,
    edge_map: &mut std::collections::HashMap<(u16, u16), u16>,
    v1: u16,
    v2: u16,
) -> u16 {
    let key = if v1 < v2 { (v1, v2) } else { (v2, v1) };

    if let Some(&existing_vertex) = edge_map.get(&key) {
        return existing_vertex;
    }

    // Create new vertex at midpoint
    let pos1 = vertices[v1 as usize].pos;
    let pos2 = vertices[v2 as usize].pos;

    let mid_x = (pos1[0] + pos2[0]) / 2.0;
    let mid_y = (pos1[1] + pos2[1]) / 2.0;
    let mid_z = (pos1[2] + pos2[2]) / 2.0;

    // Normalize to unit sphere
    let len = (mid_x * mid_x + mid_y * mid_y + mid_z * mid_z).sqrt();

    let new_vertex = vx![
        mid_x / len, mid_y / len, mid_z / len =>
        1.0, 1.0, 1.0
    ];

    vertices.push(new_vertex);
    let new_index = (vertices.len() - 1) as u16;
    edge_map.insert(key, new_index);

    new_index
}
