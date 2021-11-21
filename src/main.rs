mod creature;

fn main() {
    println!("Hello, world!");
    println!("{:?}", creature::gene::Gene::init(4, 5, 0));
}
