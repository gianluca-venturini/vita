pub mod brain;
pub mod gene;

struct Creature {
	brain: brain::Brain,
	genes: Vec<gene::Gene>,
}

impl Creature {
	fn init_random(num_internal_neurons: u8, num_genes: u8) -> Creature {
		let mut genes = Vec::new();
		for _ in 0..num_genes {
			genes.push(gene::Gene::init_random())
		}

		Creature {
			brain: brain::Brain::init(num_internal_neurons),
			genes,
		}
	}
}
