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
const NUM_INITIAL_GENE_SEQUENCES: u8 = 100;

const NUM_CREATURES: u16 = 200;
const NUM_ITERATIONS: u16 = 1000;
const NUM_GENERATIONS: u16 = 10000;

const GENERATION_TO_SAVE: u16 = 100;

fn main() {
    let mut gene_pool: Vec<Vec<creature::gene::Gene>> = Vec::new();

    // initially the gene pool is initialized randomly
    for _ in 0..NUM_INITIAL_GENE_SEQUENCES {
        let mut genes: Vec<creature::gene::Gene> = Vec::new();
        for _ in 0..NUM_GENES {
            genes.push(creature::gene::Gene::init_random())
        }
        gene_pool.push(genes);
    }

    for generation in 0..NUM_GENERATIONS {
        println!("Generation {:?}", generation);

        let mut world = world::World::init();
        let mut creatures: Vec<creature::Creature> = Vec::new();
        for _ in 0..NUM_CREATURES {
            creatures.push(creature::Creature::init_random(
                NUM_INTERNAL_NEURONS,
                &mut world,
                &gene_pool,
            ));
        }

        if generation % GENERATION_TO_SAVE == 0 {
            fs::create_dir_all(format!("./generations/{:04}", generation));
        }

        for iteration in 0..NUM_ITERATIONS {
            // println!("Iteration {:?}", iteration);

            // Optimization: don't save every generation
            if generation % GENERATION_TO_SAVE == 0 {
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
                img.save(format!(
                    "generations/{:04}/{:04}.png",
                    generation, iteration
                ))
                .unwrap();
            }

            move_all_creatures(&mut world, &mut creatures);

            // Kill creatures and extract genes of survivors
            gene_pool = get_genetic_survivors(&creatures);
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

fn get_genetic_survivors(creatures: &Vec<creature::Creature>) -> Vec<Vec<creature::gene::Gene>> {
    let mut gene_pool: Vec<Vec<creature::gene::Gene>> = Vec::new();
    for creature in creatures.iter() {
        if is_alive(&creature) {
            gene_pool.push(creature.genes.clone());
        }
    }
    if gene_pool.len() == 0 {
        println!("All creatures have died");
        panic!()
    }
    gene_pool
}

fn is_alive(creature: &creature::Creature) -> bool {
    creature.position.x > 100
}
