#[macro_use]
extern crate more_asserts;
extern crate rand;

mod creature;
mod world;

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
    move_all_creatures(&mut world, &mut creatures);
}

fn move_all_creatures(world: &mut world::World, creatures: &mut Vec<creature::Creature>) {
    world.update_creatures_positions(creatures);
    for creature in creatures.iter_mut() {
        creature.set_inputs(&world);
        creature.compute_next_state();
    }
    for creature in creatures.iter_mut() {
        let delta_position = creature.desired_move();
    }
}
