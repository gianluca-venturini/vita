pub mod brain;
pub mod gene;

use super::world;

#[derive(std::fmt::Debug)]
pub struct Creature {
	brain: brain::Brain,
	genes: Vec<gene::Gene>,
	position: world::Position,
	direction: world::Direction,
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
			position: world::Position { x: 0, y: 0 }, // TODO: how is the random position set to avoid overlapping creatures?
			direction: world::Direction::North,       // TODO: assign a random direction
		}
	}

	fn set_inputs(&mut self, world: &world::World) {
		self.brain
			.set_inputs(world, &self.position, &self.direction);
	}
}
