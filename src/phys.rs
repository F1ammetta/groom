// In a new module or file, e.g., `src/physics_object.rs`
use crate::mesh::generate_icosphere_mesh;
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
