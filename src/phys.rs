// In a new module or file, e.g., `src/physics_object.rs`
use crate::vx;
use crate::vx::Vx;
use cgmath::{Array, Quaternion, Vector3, Zero}; // Assuming you're using `glam` for vectors/quaternions

#[derive(Clone, Debug)]
pub struct Particle {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub mass: f32,
    pub radius: f32,                // For visualization and simple collision
    pub color: [f32; 3],            // For visualization
    pub acceleration: Vector3<f32>, // Can be calculated from forces
}

#[derive(Clone, Debug, Copy)]
pub struct InstanceData {
    i_pos: [f32; 3],
    i_color: [f32; 3],
    i_radius: f32,
}
implement_vertex!(InstanceData, i_pos, i_color, i_radius);

#[macro_export]
macro_rules! part {
    (
        $x:expr, $y:expr, $z:expr ; // Mandatory position
        $m:expr ;                 // Mandatory mass
        $ra:expr                  // Mandatory radius
    ) => {
        Particle {
            position: Vector3::new($x as f32, $y as f32, $z as f32),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            mass: ($m as f32),
            radius: ($ra as f32),
            color: [1.0, 1.0, 1.0],
            acceleration: Vector3::new(0.0, 0.0, 0.0),
        }
    };

    ($x:expr, $y:expr, $z:expr ;
    $vx:expr, $vy:expr, $vz:expr ;
    $m:expr ; $ra:expr ;
    $r:expr, $g:expr, $b:expr;
    $ax:expr, $ay:expr, $az:expr ) => {
        Particle {
            position: Vector3::new($x as f32, $y as f32, $z as f32),
            velocity: Vector3::new($vx as f32, $vy as f32, $vz as f32),
            mass: ($m as f32),
            radius: ($ra as f32),
            color: [$r as f32, $g as f32, $b as f32],
            acceleration: Vector3::new($ax as f32, $ay as f32, $az as f32),
        }
    };
}

// impl Particle {
//     pub fn new(
//         position: Vector3<f32>,
//         velocity: Vector3<f32>,
//         mass: f32,
//         radius: f32,
//         color: [f32; 3],
//     ) -> Self {
//         Self {
//             position,
//             velocity,
//             mass,
//             radius,
//             color,
//             acceleration: Vector3::zero(),
//         }
//     }
//
//     // You might add methods here to apply forces, etc.
// }

use std::f32::consts::PI;

pub fn generate_unit_sphere_mesh(lat_segments: u32, lon_segments: u32) -> (Vec<Vx>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices
    for lat in 0..=lat_segments {
        let theta = lat as f32 * PI / lat_segments as f32; // 0 to PI (latitude)

        for lon in 0..=lon_segments {
            let phi = lon as f32 * 2.0 * PI / lon_segments as f32; // 0 to 2*PI (longitude)

            // Spherical coordinates to Cartesian (standard physics convention)
            // x = r * sin(theta) * cos(phi)
            // y = r * cos(theta)
            // z = r * sin(theta) * sin(phi)
            let x = theta.sin() * phi.cos();
            let y = theta.cos();
            let z = theta.sin() * phi.sin();

            // Generate color based on position for visual variety
            // You can modify this to use any color scheme you prefer
            let r = (x + 1.0) * 0.5; // Normalize -1..1 to 0..1
            let g = (y + 1.0) * 0.5;
            let b = (z + 1.0) * 0.5;

            vertices.push(vx![x, y, z => r, g, b]);
        }
    }

    // Generate indices for triangles, avoiding degenerate triangles at poles
    for lat in 0..lat_segments {
        for lon in 0..lon_segments {
            let current = lat * (lon_segments + 1) + lon;
            let next = current + lon_segments + 1;

            // Skip degenerate triangles at the poles
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

// Alternative implementation using icosphere subdivision for better triangle distribution
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

pub struct PhysicsWorld {
    pub particles: Vec<Particle>,
    pub gravity: Vector3<f32>,
    // Store the base sphere mesh
    base_sphere_vertices: Vec<Vx>,
    base_sphere_indices: Vec<u16>,
    // Store the offset for indices when combining meshes
    sphere_vertex_count: u32,
    sphere_index_count: u32,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        // let (verts, inds) = generate_unit_sphere_mesh(1024, 16); // Choose segments for desired smoothness
        // let (verts, inds) = generate_unit_sphere_mesh(16, 32);
        let (verts, inds) = generate_icosphere_mesh(2);
        let vert_count = verts.len() as u32;
        let index_count = inds.len() as u32;

        Self {
            particles: Vec::new(),
            gravity: Vector3::new(0.0, -9.81, 0.0),
            base_sphere_vertices: verts,
            base_sphere_indices: inds,
            sphere_vertex_count: vert_count,
            sphere_index_count: index_count,
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    // New method to get instance data instead of all vertices
    pub fn get_instance_data(&self) -> Vec<InstanceData> {
        self.particles
            .iter()
            .map(|p| InstanceData {
                i_pos: p.position.into(),
                i_color: p.color,
                i_radius: p.radius,
            })
            .collect()
    }

    // Keep base mesh separate
    pub fn get_base_mesh(&self) -> (&Vec<Vx>, &Vec<u16>) {
        (&self.base_sphere_vertices, &self.base_sphere_indices)
    }

    /// The core update function for the simulation
    pub fn update(&mut self, dt: f32) {
        for particle in &mut self.particles {
            // 1. Calculate Net Force (e.g., gravity)
            let mut net_force = self.gravity * particle.mass;
            // Add other forces here (e.g., drag, springs, user input)

            // 2. Calculate Acceleration
            particle.acceleration = net_force / particle.mass;

            // 3. Update Velocity (Euler integration)
            particle.velocity += particle.acceleration * dt;

            // 4. Update Position (Euler integration)
            particle.position += particle.velocity * dt;

            // Simple "ground" collision to prevent falling indefinitely
            if particle.position.y < particle.radius {
                particle.position.y = particle.radius;
                // Reflect velocity and apply damping
                particle.velocity.y *= -0.8; // Lose 20% energy
            }
        }
        // You would also handle inter-particle collisions here
    }
}
