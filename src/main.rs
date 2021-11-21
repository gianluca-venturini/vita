#[macro_use]
extern crate more_asserts;

mod creature;

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
}
