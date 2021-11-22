#[macro_use]
extern crate more_asserts;
extern crate image;
extern crate rand;
use std::fs;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

mod creature;
mod world;

const NUM_INTERNAL_NEURONS: u8 = 1;
const NUM_GENES: u8 = 10;

fn main() {
    println!("Hello, world!");
    println!(
        "{}",
        creature::gene::Gene::init(
            creature::brain::NeuronLayer::Input,
            0,
            creature::brain::NeuronLayer::Internal,
            0,
            0
        )
    );
    println!(
        "{}",
        creature::gene::Gene::init(
            creature::brain::NeuronLayer::Internal,
            127,
            creature::brain::NeuronLayer::Output,
            127,
            -1
        )
    );
    let mut world = world::World::init();
    let mut creatures: Vec<creature::Creature> = Vec::new();
    for _ in 0..100 {
        creatures.push(creature::Creature::init_random(
            NUM_INTERNAL_NEURONS,
            NUM_GENES,
            &mut world,
        ));
    }

    for generation in 0..2 {
        fs::create_dir_all(format!("./generations/{:5}", generation));
        for iteration in 0..100 {
            println!("Iteration {:?}", iteration);
            let img = ImageBuffer::from_fn(128, 128, |x, y| {
                if world.coordinates.contains_key(&world::Position {
                    x: x as u16,
                    y: y as u16,
                }) {
                    image::Luma([0u8])
                } else {
                    image::Luma([255u8])
                }
            });
            img.save(format!("generations/{:5}/{:5}.png", 0, iteration))
                .unwrap();
            for creature in creatures.iter() {
                println!("{}", creature);
            }
            move_all_creatures(&mut world, &mut creatures);
        }
    }
}

fn move_all_creatures(world: &mut world::World, creatures: &mut Vec<creature::Creature>) {
    for creature in creatures.iter_mut() {
        creature.set_inputs(&world);
        creature.compute_next_state();
    }
    for creature in creatures.iter_mut() {
        world.move_creature(creature);
    }
}
