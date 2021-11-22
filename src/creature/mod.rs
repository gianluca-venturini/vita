use super::world;
use std::fmt::{self, Debug, Display, Formatter};

pub mod brain;
pub mod gene;

#[derive(Debug, Clone)]
pub struct Creature {
	brain: brain::Brain,
	genes: Vec<gene::Gene>,
	pub position: world::Position,
	direction: world::Direction,
}

impl Display for Creature {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"{}: {:?}: {:?}",
			self.genes
				.iter()
				.map(|gene| format!("{}", gene))
				.collect::<Vec<String>>()
				.join(" "),
			self.position,
			self.direction,
		)
	}
}

impl Creature {
	pub fn init_random(num_internal_neurons: u8, num_genes: u8) -> Creature {
		let mut genes = Vec::new();
		for _ in 0..num_genes {
			genes.push(gene::Gene::init_random())
		}

		Creature {
			brain: brain::Brain::init(num_internal_neurons),
			genes,
			position: world::Position { x: 0, y: 0 }, // TODO: how is the random position set to avoid overlapping creatures?
			direction: rand::random(),
		}
	}

	pub fn set_inputs(&mut self, world: &world::World) {
		self.brain
			.set_inputs(world, &self.position, &self.direction);
	}

	pub fn compute_next_state(&mut self) {
		self.brain.compute_neurons_state(&self.genes);
	}

	pub fn desired_move(&self) -> world::DeltaPosition {
		self.brain.desired_move(&self.direction)
	}
}
