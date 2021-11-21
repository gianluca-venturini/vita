mod creature;

fn main() {
    println!("Hello, world!");
    println!("{}", creature::gene::Gene::init(1, 1, 1));
    println!("{}", creature::gene::Gene::init(255, 255, -1));
}
