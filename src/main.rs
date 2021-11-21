mod gene;

fn main() {
    println!("Hello, world!");
    println!("{:?}", gene::Gene::init(4, 5, 0));
}
