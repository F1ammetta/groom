use std::fmt::Display;

use crate::mesh::generate_icosphere_mesh;
use crate::vx::Vx;
use cgmath::{ElementWise, InnerSpace, Vector3, Vector4, Zero};
use glium::implement_vertex;

#[derive(Clone, Debug)]
pub struct Particle {
    pub position: Vector4<f32>,
    pub velocity: Vector4<f32>,
    pub v: Vector4<f32>,
    pub acceleration: Vector4<f32>, // Can be calculated from forces
    pub mass: f32,
    pub radius: f32,     // For visualization and simple collision
    pub color: [f32; 3], // For visualization
    pub tau: f32,
}

#[derive(Clone, Debug)]
pub struct Plane {
    pub verts: Vec<Vector3<f32>>,
    pub flat: bool,
    pub color: [f32; 3],
}

pub fn get_plane_verts(
    x: f32,
    y: f32,
    z: f32,
    xl: f32,
    yl: f32,
    zl: f32,
) -> (Vec<Vector3<f32>>, bool) {
    let mut flat = true;
    let mut ax1: Vector3<f32> = Vector3::unit_x();
    let mut ax2: Vector3<f32> = Vector3::unit_y();
    let mut l1: f32 = 0.0;
    let mut l2: f32 = 0.0;

    let center = Vector3::new(x, y, z);

    match (xl == 0.0, yl == 0.0, zl == 0.0) {
        (true, false, false) => {
            ax1 = Vector3::unit_y();
            ax2 = Vector3::unit_z();
            l1 = yl;
            l2 = zl;
        }
        (false, true, false) => {
            ax1 = Vector3::unit_x();
            ax2 = Vector3::unit_z();
            l1 = xl;
            l2 = zl;
        }
        (false, false, true) => {
            ax1 = Vector3::unit_x();
            ax2 = Vector3::unit_y();
            l1 = xl;
            l2 = yl;
        }
        (false, false, false) => {
            flat = false;
        }
        _ => return (Vec::new(), true),
    }

    let mut verts: Vec<Vector3<f32>> = Vec::new();

    if flat {
        verts.push(center + ax1 * l1 / 2.0);
        verts.push(center + ax2 * l2 / 2.0);
        verts.push(center - ax1 * l1 / 2.0);
        verts.push(center - ax2 * l2 / 2.0);
    } else {
        verts.push(center + (Vector3::unit_x() * xl / 2.0) + (Vector3::unit_z() * zl / 2.0));
        verts.push(center - (Vector3::unit_x() * xl / 2.0) + (Vector3::unit_z() * zl / 2.0));
        verts.push(center + (Vector3::unit_x() * xl / 2.0) - (Vector3::unit_z() * zl / 2.0));
        verts.push(center - (Vector3::unit_x() * xl / 2.0) - (Vector3::unit_z() * zl / 2.0));
        verts.push(center + (Vector3::unit_y() * yl / 2.0) + (Vector3::unit_z() * zl / 2.0));
        verts.push(center - (Vector3::unit_y() * yl / 2.0) + (Vector3::unit_z() * zl / 2.0));
        verts.push(center + (Vector3::unit_y() * yl / 2.0) - (Vector3::unit_z() * zl / 2.0));
        verts.push(center - (Vector3::unit_y() * yl / 2.0) - (Vector3::unit_z() * zl / 2.0));
    }

    (verts, flat)
}

#[macro_export]
macro_rules! plane {
    (
        $x:expr, $y:expr, $z:expr ; // Center
        $xl:expr, $yl:expr, $zl:expr;
        $rgb:expr
    ) => {
        match get_plane_verts(
            $x as f32, $y as f32, $z as f32, $xl as f32, $yl as f32, $zl as f32,
        ) {
            (verts, flat) => Plane {
                verts,
                flat,
                color: $rgb,
            },
        }
    };
}

impl Display for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pos: [{}, {}, {}, {}] Vel: [{}, {}, {}, {}]",
            self.position[0] / C,
            self.position[1],
            self.position[2],
            self.position[3],
            self.velocity[0] / C,
            self.velocity[1],
            self.velocity[2],
            self.velocity[3],
        )
    }
}

impl Particle {
    pub fn new(
        position: Vector4<f32>,
        v: Vector4<f32>,
        mass: f32,
        radius: f32,
        color: [f32; 3],
        tau: f32,
    ) -> Self {
        let u = normalize_4v(Vector4::new(C, v[1], v[2], v[3]));
        Particle {
            position,
            velocity: u,
            v,
            acceleration: Vector4::zero(),
            mass,
            radius,
            color,
            tau,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct InstanceData {
    i_pos: [f32; 3],
    i_color: [f32; 3],
    i_radius: f32,
}
implement_vertex!(InstanceData, i_pos, i_color, i_radius);

pub const C: f32 = 299792458.0;

#[macro_export]
macro_rules! part {
    (
        $t:expr, $x:expr, $y:expr, $z:expr ; // Mandatory position
        $m:expr ;                 // Mandatory mass
        $ra:expr                  // Mandatory radius
    ) => {
        Particle::new(
            Vector4::new($t as f32, $x as f32, $y as f32, $z as f32),
            Vector4::new(C, 0.0, 0.0, 0.0),
            ($m as f32),
            ($ra as f32),
            [1.0, 1.0, 1.0],
            0.0,
        )
    };

    ($t:expr, $x:expr, $y:expr, $z:expr ;
    $vx:expr, $vy:expr, $vz:expr ;
    $m:expr ; $ra:expr ;
    $r:expr, $g:expr, $b:expr) => {
        Particle::new(
            Vector4::new($t as f32, $x as f32, $y as f32, $z as f32),
            Vector4::new(C, $vx as f32, $vy as f32, $vz as f32),
            ($m as f32),
            ($ra as f32),
            [$r as f32, $g as f32, $b as f32],
            0.0,
        )
    };
}

pub struct PhysicsWorld {
    pub particles: Vec<Particle>,
    pub planes: Vec<Plane>,
    pub gravity: Vector4<f32>,
    t: f32,
    base_sphere_vertices: Vec<Vx>,
    base_sphere_indices: Vec<u16>,
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
            planes: Vec::new(),
            gravity: Vector4::new(0.0, 0.0, 0.0, 0.0),
            t: 0.0,
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
                i_pos: [p.position[1], p.position[2], p.position[3]],
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
    // pub fn update(&mut self, dt: f32) {
    //     for p in self.particles.iter_mut() {
    //         let gamma = p.velocity[0] / C;
    //
    //         let dtau = dt / gamma;
    //         p.tau += dtau;
    //
    //         p.v += p.acceleration * dt;
    //
    //         p.velocity = normalize_4v(p.v);
    //
    //         p.position += p.velocity * dt;
    //         p.position[0] = (self.t + dt) * C;
    //     }
    //     self.t += dt;
    //     // You would also handle inter-particle collisions here
    // }
    pub fn update(&mut self, dt: f32) {
        let num_particles = self.particles.len();

        // Phase 1: Update 3-velocity (`p.v`) based on acceleration for all particles.
        // This calculates the velocity *before* any collisions in this timestep.
        for p in self.particles.iter_mut() {
            // Apply acceleration to 3-velocity using cgmath vector operations
            p.v += p.acceleration * dt;
        }

        // Phase 2: Handle inter-particle collisions.
        // We'll compute new velocities into a temporary buffer `new_vs`
        // to avoid mutable borrowing conflicts and order-of-collision dependencies.
        let mut new_vs: Vec<Vector3<f32>> = self
            .particles
            .iter()
            .map(|p| Vector3::new(p.v[1], p.v[2], p.v[3]))
            .collect();

        for i in 0..num_particles {
            for j in (i + 1)..num_particles {
                // Extract 3D spatial position from 4D position Vector4
                let p1_pos_spatial = Vector3::new(
                    self.particles[i].position[1],
                    self.particles[i].position[2],
                    self.particles[i].position[3],
                );
                let p2_pos_spatial = Vector3::new(
                    self.particles[j].position[1],
                    self.particles[j].position[2],
                    self.particles[j].position[3],
                );

                // Use velocities from the `new_vs` buffer as they might have been
                // adjusted by previous collisions in this same timestep.
                let p1_v = new_vs[i];
                let p2_v = new_vs[j];

                let p1_mass = self.particles[i].mass;
                let p2_mass = self.particles[j].mass;
                let p1_radius = self.particles[i].radius;
                let p2_radius = self.particles[j].radius;

                // Relative position vector (from p2 to p1)
                let relative_pos = p1_pos_spatial - p2_pos_spatial;
                let dist_sq = relative_pos.magnitude2(); // Squared distance
                let radius_sum = p1_radius + p2_radius;

                // Check for collision: if distance <= sum of radii
                // Also ensure distance is not near zero to prevent division by zero for normal vector.
                if dist_sq <= radius_sum * radius_sum && dist_sq > 1e-6 {
                    // Use small epsilon to avoid exactly zero distance
                    let dist = dist_sq.sqrt();

                    // Overlap correction: Slightly separate particles if they are too close/overlapping.
                    let overlap = radius_sum - dist;
                    if overlap > 0.0 {}

                    // Collision normal vector (points from p2 to p1)
                    let normal = relative_pos.normalize(); // Use cgmath's normalize method

                    // Relative velocity (p1_v - p2_v)
                    let relative_velocity = p1_v - p2_v;

                    // Relative velocity along the collision normal
                    let vel_along_normal = relative_velocity.dot(normal); // Use cgmath's dot method

                    // Only resolve if particles are moving towards each other (closing in)
                    if vel_along_normal < 0.0 {
                        // Coefficient of restitution (e = 1.0 for perfectly elastic collision)
                        let e = 1.0;

                        // Calculate impulse magnitude (j)
                        // j = -(1 + e) * vel_along_normal / (1/m1 + 1/m2)
                        let impulse_magnitude =
                            -(1.0 + e) * vel_along_normal / (1.0 / p1_mass + 1.0 / p2_mass);

                        // Apply impulse to update velocities
                        // Delta_v = impulse_magnitude * normal / mass
                        new_vs[i] += normal * (impulse_magnitude / p1_mass);
                        new_vs[j] -= normal * (impulse_magnitude / p2_mass); // p2 gets impulse in opposite direction
                    }
                }
            }
        }

        // Apply the updated 3-velocities back to the particles
        for i in 0..num_particles {
            self.particles[i].v = Vector4::new(C, new_vs[i][0], new_vs[i][1], new_vs[i][2]);
        }

        // Phase 3: Update 4-velocity, position, and proper time for all particles.
        // This uses the final 3-velocities after collision resolution.
        for p in self.particles.iter_mut() {
            // Recompute the 4-velocity based on the updated 3-velocity (`p.v`)
            p.velocity = normalize_4v(p.v);

            // Update proper time (`tau`) using the new gamma from `p.velocity`
            // Assuming p.velocity[0] holds gamma * C.
            let gamma = p.velocity[0] / C; // Using p.velocity[0] to access the first component (ct)
            let dtau = dt / gamma;
            p.tau += dtau;

            // Update spatial position components using the 4-velocity
            p.position[1] += p.velocity[1] * dt; // x component
            p.position[2] += p.velocity[2] * dt; // y component
            p.position[3] += p.velocity[3] * dt; // z component

            // Update the time component of the 4-position.
            // This assumes a global time coordinate `t` for the simulation.
            p.position[0] = (self.t + dt) * C;
        }

        // Advance the global simulation time
        self.t += dt;
    }
}

fn normalize_4v(v: Vector4<f32>) -> Vector4<f32> {
    let v_sq = v[1].powi(2) + v[2].powi(2) + v[3].powi(2);
    let gamma = 1.0 / (1.0 - (v_sq / (C.powi(2)))).sqrt();

    v * gamma
}
