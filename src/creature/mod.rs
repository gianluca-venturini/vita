use super::world;
use rand::prelude::*;
use std::fmt::{self, Debug, Display, Formatter};

pub mod brain;
pub mod gene;

const MUTATION_CHANCE: f32 = 0.1f32;

#[derive(Debug, Clone)]
pub struct Creature {
	pub brain: brain::Brain,
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
	pub fn init_random(
		num_internal_neurons: u8,
		num_genes: u8,
		world: &mut world::World,
		gene_pool: &Vec<Vec<gene::Gene>>,
	) -> Creature {
		let mut rng = rand::thread_rng();
		let r: u16 = rng.gen();

		// Get a random set of genes from the gene pool
		let mut genes = gene_pool
			.get((r % gene_pool.len() as u16) as usize)
			.unwrap()
			.clone();

		let mut rng = rand::thread_rng();
		let mut position: world::Position;
		loop {
			let rx: u16 = rng.gen();
			let ry: u16 = rng.gen();
			position = world::Position {
				x: rx % world.boundary.width,
				y: ry % world.boundary.height,
			};
			if !world.coordinates.contains_key(&position) {
				break;
			}
		}

		let creature = Creature {
			brain: brain::Brain::init(num_internal_neurons),
			genes,
			position,
			direction: rand::random(),
		};
		world
			.coordinates
			.insert(creature.position, creature.clone());
		creature
	}

	pub fn init(num_internal_neurons: u8, num_genes: u8) -> Creature {
		let mut genes = Vec::new();
		for _ in 0..num_genes {
			genes.push(gene::Gene::init(
				brain::NeuronLayer::Input,
				0,
				brain::NeuronLayer::Internal,
				0,
				0,
			))
		}

		Creature {
			brain: brain::Brain::init(num_internal_neurons),
			genes,
			position: world::Position { x: 0, y: 0 },
			direction: world::Direction::North,
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

	pub fn get_repro_genetic(&self) -> Vec<gene::Gene> {
		let mut rng = rand::thread_rng();

		let mut genes: Vec<gene::Gene> = Vec::new();
		for gene in self.genes.iter() {
			let r: f32 = rng.gen();
			if r < MUTATION_CHANCE {
				let mutation: u8 = rng.gen();
				let mut new_gene = gene.clone();
				new_gene.mutate(mutation % 32);
				genes.push(new_gene);
			} else {
				genes.push(gene.clone());
			}
		}
		genes
	}
}
