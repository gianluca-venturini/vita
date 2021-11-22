#[macro_use]
extern crate more_asserts;
extern crate rand;

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
    for i in 0..2 {
        println!("Iteration {:?}", i);
        for creature in creatures.iter() {
            println!("{}", creature);
        }
        move_all_creatures(&mut world, &mut creatures);
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
