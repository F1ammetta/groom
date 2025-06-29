use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};

use cgmath::{Vector3, Vector4};
use crossbeam::channel::Sender;

use crate::part;
use crate::phys::C;
use crate::phys::get_plane_verts;
use crate::phys::{InstanceData, PhysicsWorld};
use crate::phys::{Particle, Plane};
use crate::plane;

pub enum PhysicsMessage {
    InstanceData(Vec<InstanceData>),
}

pub fn phys_start(running: Arc<AtomicBool>, tx: Sender<PhysicsMessage>) {
    let mut world = PhysicsWorld::new();

    world.add_particle(part![
            0.0,0.0,5.0,100.0;
            0.0,0.0,-3000.0;
            1;
            10;
            0.0,0.6,0.8
    ]);

    world.add_particle(part![
            0.0,0.0,5.0,-100.0;
            0.0,0.0,3000.0;
            1;
            10;
            0.5,0.6,0.8
    ]);

    world.add_particle(part![
            0.0,50.0,5.0,0.0;
            -1500.0,0.0,0.0;
            1;
            10;
            0.2,0.4,0.8
    ]);

    let mut lt = Instant::now();

    while running.load(Ordering::SeqCst) {
        let dt = lt.elapsed().as_secs_f32();
        lt = Instant::now();

        world.update(dt);

        // println!("freq: {} Hz", 1.0 / dt);

        match tx.send(PhysicsMessage::InstanceData(world.get_instance_data())) {
            Ok(_) => {}
            Err(e) => println!("Failed to communicate from physics thread: {:?}", e),
        };
    }
}
