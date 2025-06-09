use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use cgmath::Vector3;
use crossbeam::channel::Sender;

use crate::part;
use crate::phys::Particle;
use crate::phys::{InstanceData, PhysicsWorld};

pub enum PhysicsMessage {
    InstanceData(Vec<InstanceData>),
}

pub fn phys_start(running: Arc<AtomicBool>, tx: Sender<PhysicsMessage>) {
    let mut world = PhysicsWorld::new();

    world.add_particle(part![0.0,0.0,0.0;1;0.3]);

    while running.load(Ordering::SeqCst) {
        tx.send(PhysicsMessage::InstanceData(world.get_instance_data()))
            .unwrap();
    }
}
