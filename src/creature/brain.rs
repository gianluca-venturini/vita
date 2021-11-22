use super::gene::Gene;
use super::world;
use super::Creature;
use rand::prelude::*;
use std::cmp;
use std::collections::HashMap;

pub struct BrainDescription {
	pub num_input: u8,
	pub num_internal: u8,
	pub num_output: u8,
}

impl BrainDescription {
	fn init(num_internal: u8) -> BrainDescription {
		BrainDescription {
			num_internal,
			num_input: INPUT_NEURONS.len() as u8,
			num_output: OUTPUT_NEURONS.len() as u8,
		}
	}
}

#[derive(Debug)]
pub struct Brain {
	input: Vec<Neuron>,
	internal: Vec<Neuron>,
	output: Vec<Neuron>,
}

impl Clone for Brain {
	fn clone(&self) -> Brain {
		Brain {
			input: self.input.clone(),
			internal: self.internal.clone(),
			output: self.output.clone(),
		}
	}
}

impl Brain {
	pub fn init(num_internal: u8) -> Brain {
		Brain {
			input: INPUT_NEURONS
				.to_vec()
				.into_iter()
				.map(|neuron_type| Neuron {
					neuron_type,
					value: 0f32,
					neuron_layer: NeuronLayer::Input,
				})
				.collect(),
			output: OUTPUT_NEURONS
				.to_vec()
				.into_iter()
				.map(|neuron_type| Neuron {
					neuron_type,
					value: 0f32,
					neuron_layer: NeuronLayer::Output,
				})
				.collect(),
			internal: vec![
				Neuron {
					neuron_type: NeuronType::Internal,
					value: 0f32,
					neuron_layer: NeuronLayer::Internal,
				};
				num_internal as usize
			],
		}
	}

	pub fn compute_neurons_state(&mut self, genes: &Vec<Gene>) {
		// Reset all neurons
		self.reset_neurons_layer(NeuronLayer::Internal);
		self.reset_neurons_layer(NeuronLayer::Output);
		let connections = self.get_connection_from_genes(genes);

		// Compute all neurons with input layer source
		self.compute_normalized_sum_on_destination_neurons(
			&connections,
			NeuronLayer::Input,
			NeuronLayer::Internal,
		);
		self.compute_normalized_sum_on_destination_neurons(
			&connections,
			NeuronLayer::Input,
			NeuronLayer::Output,
		);

		// Compute all internal neurons that are connected to intermediate neurons
		self.compute_normalized_sum_on_destination_neurons(
			&connections,
			NeuronLayer::Internal,
			NeuronLayer::Internal,
		);

		// Compute all neurons with intermediate layer source
		self.compute_normalized_sum_on_destination_neurons(
			&connections,
			NeuronLayer::Internal,
			NeuronLayer::Output,
		);
	}

	pub fn set_inputs(
		&mut self,
		world: &world::World,
		position: &world::Position,
		direction: &world::Direction,
	) {
		for neuron in self.input.iter_mut() {
			neuron.set_from_world(world, position, direction)
		}
	}

	pub fn desired_move(&self, direction: &world::Direction) -> world::DeltaPosition {
		let mut delta = world::DeltaPosition { x: 0f32, y: 0f32 };
		for neuron in self.output.iter() {
			let delta_neuron = neuron.desired_move(direction);
			delta.x += delta_neuron.x;
			delta.y += delta_neuron.y;
		}
		delta
	}

	fn reset_neurons_layer(&mut self, layer: NeuronLayer) {
		let neurons = self.get_neurons_layer(layer);
		for neuron in neurons.iter_mut() {
			neuron.value = 0f32;
		}
	}

	fn get_neurons_layer(&mut self, layer: NeuronLayer) -> &mut Vec<Neuron> {
		match layer {
			NeuronLayer::Input => &mut self.input,
			NeuronLayer::Internal => &mut self.internal,
			NeuronLayer::Output => &mut self.output,
		}
	}

	fn compute_normalized_sum_on_destination_neurons(
		&mut self,
		connections: &Vec<NeuronConnection>,
		source_layer: NeuronLayer,
		destination_layer: NeuronLayer,
	) {
		// Accumulate all the changes in a separate area to ensure
		// that the result of computations at this step are not counted
		// as input for the following elements
		let mut changes: HashMap<u8, f32> = HashMap::new();
		for connection in connections.iter() {
			if connection.source.neuron_layer == source_layer
				&& connection.destination.neuron_layer == destination_layer
			{
				let weighted_value: f32;
				{
					let source = self.desc_to_neuron(&connection.source);
					weighted_value = source.value * connection.weight;
				}
				changes.insert(
					// accumulate the value
					connection.destination.neuron_number,
					match changes.get(&connection.destination.neuron_number) {
						Some(x) => *x,
						None => 0f32,
					} + weighted_value,
				);
			}
		}
		// Now is safe to apply the changes
		for (neuron_number, value_change) in changes.iter_mut() {
			let neuron = self.desc_to_neuron(&NeuronDescription {
				neuron_number: *neuron_number,
				neuron_layer: destination_layer,
			});
			neuron.value = (neuron.value.atanh() + *value_change).tanh();
		}
	}

	fn desc_to_neuron(&mut self, desc: &NeuronDescription) -> &mut Neuron {
		match desc.neuron_layer {
			NeuronLayer::Input => &mut self.input[desc.neuron_number as usize],
			NeuronLayer::Internal => &mut self.internal[desc.neuron_number as usize],
			NeuronLayer::Output => &mut self.output[desc.neuron_number as usize],
		}
	}

	fn get_connection_from_genes(&self, genes: &Vec<Gene>) -> Vec<NeuronConnection> {
		let mut connections: Vec<NeuronConnection> = Vec::new();

		for gene in genes {
			let source = gene.get_source_neuron(&self.to_brain_description());
			let destination = gene.get_destination_neuron(&self.to_brain_description());
			// weight is scaled for having smaller numbers
			// and being able to follow the calculations by hand
			// if something goes wrong
			let weight = f32::from(gene.weight) / 8192f32;
			connections.push(NeuronConnection {
				source,
				destination,
				weight,
			})
		}

		connections
	}

	fn to_brain_description(&self) -> BrainDescription {
		BrainDescription {
			num_input: self.input.len() as u8,
			num_internal: self.internal.len() as u8,
			num_output: self.output.len() as u8,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum NeuronLayer {
	Input,
	Internal,
	Output,
}

#[derive(Clone, Copy, std::hash::Hash, Debug)]
pub enum NeuronType {
	// Input
	Random,
	BlockLeftRight,
	BlockForward,
	LastMovementY,
	LastMovementX,
	BorderDistanceNorthSouth,
	BorderDistanceEastWest,
	WordLocationNorthSouth,
	WordLocationEastWest,

	// Internal
	Internal,

	// Output
	MoveForward,
	MoveRandom,
	MoveReverse,
	MoveLeftRight,
	MoveEastWest,
	MoveNorthSouth,
}

const INPUT_NEURONS: [NeuronType; 9] = [
	NeuronType::Random,
	NeuronType::BlockLeftRight,
	NeuronType::BlockForward,
	NeuronType::LastMovementY,
	NeuronType::LastMovementX,
	NeuronType::BorderDistanceNorthSouth,
	NeuronType::BorderDistanceEastWest,
	NeuronType::WordLocationNorthSouth,
	NeuronType::WordLocationEastWest,
];

const OUTPUT_NEURONS: [NeuronType; 6] = [
	NeuronType::MoveForward,
	NeuronType::MoveRandom,
	NeuronType::MoveReverse,
	NeuronType::MoveLeftRight,
	NeuronType::MoveEastWest,
	NeuronType::MoveNorthSouth,
];

#[derive(Debug, PartialEq)]
pub struct NeuronDescription {
	pub neuron_layer: NeuronLayer,
	pub neuron_number: u8,
}

#[derive(Debug, Clone)]
pub struct Neuron {
	neuron_type: NeuronType,
	neuron_layer: NeuronLayer,
	value: f32,
}

impl Neuron {
	pub fn fire(&self) {
		// This threshold is somewhat arbitrary. TODO: tweak
		self.value > 0.5f32;
	}

	pub fn set_from_world(
		&mut self,
		world: &world::World,
		position: &world::Position,
		direction: &world::Direction,
	) {
		match self.neuron_type {
			NeuronType::Random => {
				let mut rng = rand::thread_rng();
				let random_number: f32 = rng.gen(); // Generated number uniformly distributed [0, 1)
				self.value = random_number * 2.0 - 1.0;
			}
			NeuronType::BlockLeftRight => {
				let right = position.move_direction(&direction.rotate_right(), 1, &world.boundary);
				let left = position.move_direction(&direction.rotate_left(), 1, &world.boundary);
				if (right.is_some() && world.coordinates.contains_key(&right.unwrap()))
					|| (left.is_some() && world.coordinates.contains_key(&left.unwrap()))
				{
					self.value = 1f32;
				} else {
					self.value = 0f32;
				}
			}
			NeuronType::BlockForward => {
				let forward = position.move_direction(direction, 1, &world.boundary);
				if forward.is_some() && world.coordinates.contains_key(&forward.unwrap()) {
					self.value = 1f32;
				} else {
					self.value = 0f32;
				}
			}
			// TODO: finish implementing the other input neurons
			NeuronType::LastMovementY => {}
			NeuronType::LastMovementX => {}
			NeuronType::BorderDistanceNorthSouth => {}
			NeuronType::BorderDistanceEastWest => {}
			NeuronType::WordLocationNorthSouth => {}
			NeuronType::WordLocationEastWest => {}

			NeuronType::Internal => {}

			NeuronType::MoveForward => {}
			NeuronType::MoveRandom => {}
			NeuronType::MoveReverse => {}
			NeuronType::MoveLeftRight => {}
			NeuronType::MoveEastWest => {}
			NeuronType::MoveNorthSouth => {}
		};
	}

	pub fn desired_move(&self, direction: &world::Direction) -> world::DeltaPosition {
		match self.neuron_type {
			NeuronType::Random => world::DeltaPosition { x: 0f32, y: 0f32 },
			NeuronType::BlockLeftRight => world::DeltaPosition { x: 0f32, y: 0f32 },
			NeuronType::BlockForward => world::DeltaPosition { x: 0f32, y: 0f32 },
			NeuronType::LastMovementY => world::DeltaPosition { x: 0f32, y: 0f32 },
			NeuronType::LastMovementX => world::DeltaPosition { x: 0f32, y: 0f32 },
			NeuronType::BorderDistanceNorthSouth => world::DeltaPosition { x: 0f32, y: 0f32 },
			NeuronType::BorderDistanceEastWest => world::DeltaPosition { x: 0f32, y: 0f32 },
			NeuronType::WordLocationNorthSouth => world::DeltaPosition { x: 0f32, y: 0f32 },
			NeuronType::WordLocationEastWest => world::DeltaPosition { x: 0f32, y: 0f32 },

			NeuronType::Internal => world::DeltaPosition { x: 0f32, y: 0f32 },

			// TODO: Implement output neurons
			NeuronType::MoveForward => world::DeltaPosition { x: 0f32, y: 0f32 }
				.move_direction(direction, self.value.max(0f32)),
			NeuronType::MoveRandom => {
				let mut rng = rand::thread_rng();
				world::DeltaPosition {
					x: rng.gen(),
					y: rng.gen(),
				}
			}
			NeuronType::MoveReverse => world::DeltaPosition { x: 0f32, y: 0f32 }
				.move_direction(&direction.rotate_left().rotate_left(), self.value.max(0f32)),
			NeuronType::MoveLeftRight => world::DeltaPosition { x: 0f32, y: 0f32 }
				.move_direction(&direction.rotate_right(), self.value),
			NeuronType::MoveEastWest => world::DeltaPosition {
				x: self.value,
				y: 0f32,
			},
			NeuronType::MoveNorthSouth => world::DeltaPosition {
				x: 0f32,
				y: self.value,
			},
		}
	}
}

#[derive(Debug)]
struct NeuronConnection {
	source: NeuronDescription,
	destination: NeuronDescription,
	weight: f32,
}

// Small value used to keep into account inaccuracies
const EPSILON: f32 = 0.01f32;

#[test]
fn should_compute_single_connection_input_internal_positive_weight() {
	let mut brain = Brain::init(2);
	let genes = Vec::from([Gene::init(
		NeuronLayer::Input,
		0,
		NeuronLayer::Internal,
		0,
		32767i16,
	)]);
	assert_eq!(brain.output[0].value, 0f32);
	brain.input[0].value = 1f32;
	brain.compute_neurons_state(&genes);
	assert_eq!(brain.input[0].value, 1f32);
	assert_gt!(brain.internal[0].value, 1f32 - EPSILON);
	assert_eq!(brain.internal[1].value, 0f32);
}
#[test]
fn should_compute_single_connection_input_internal_positive_small_weight() {
	let mut brain = Brain::init(2);
	let genes = Vec::from([Gene::init(
		NeuronLayer::Input,
		0,
		NeuronLayer::Internal,
		0,
		1i16,
	)]);
	assert_eq!(brain.output[0].value, 0f32);
	brain.input[0].value = 1f32;
	brain.compute_neurons_state(&genes);
	assert_eq!(brain.input[0].value, 1f32);
	assert_gt!(brain.internal[0].value, 0f32);
}

#[test]
fn should_compute_single_connection_input_internal_negative_weight() {
	let mut brain = Brain::init(2);
	let genes = Vec::from([Gene::init(
		NeuronLayer::Input,
		0,
		NeuronLayer::Internal,
		0,
		-32766i16,
	)]);
	assert_eq!(brain.output[0].value, 0f32);
	brain.input[0].value = 1f32;
	brain.compute_neurons_state(&genes);
	assert_eq!(brain.input[0].value, 1f32);
	assert_lt!(brain.internal[0].value, -1f32 + EPSILON);
}

#[test]
fn should_compute_single_connection_input_output() {
	let mut brain = Brain::init(2);
	let genes = Vec::from([Gene::init(
		NeuronLayer::Input,
		0,
		NeuronLayer::Output,
		0,
		32767i16,
	)]);
	assert_eq!(brain.output[0].value, 0f32);
	brain.input[0].value = 1f32;
	brain.compute_neurons_state(&genes);
	assert_eq!(brain.input[0].value, 1f32);
	assert_gt!(brain.output[0].value, 1f32 - EPSILON);
}

#[test]
fn should_compute_two_connections_internal_intermediate_output() {
	let mut brain = Brain::init(2);
	let genes = Vec::from([
		Gene::init(NeuronLayer::Input, 0, NeuronLayer::Internal, 0, 32767i16),
		Gene::init(NeuronLayer::Internal, 0, NeuronLayer::Output, 0, 32767i16),
	]);
	assert_eq!(brain.output[0].value, 0f32);
	brain.input[0].value = 1f32;
	brain.compute_neurons_state(&genes);
	assert_eq!(brain.input[0].value, 1f32);
	assert_gt!(brain.internal[0].value, 1f32 - EPSILON);
	assert_gt!(brain.output[0].value, 1f32 - EPSILON);
}

#[test]
fn should_compute_internal_connected_two_output() {
	let mut brain = Brain::init(2);
	let genes = Vec::from([
		Gene::init(NeuronLayer::Input, 0, NeuronLayer::Internal, 0, 32767i16),
		Gene::init(NeuronLayer::Internal, 0, NeuronLayer::Output, 0, 32767i16),
		Gene::init(NeuronLayer::Internal, 0, NeuronLayer::Output, 1, 32767i16),
	]);
	assert_eq!(brain.output[0].value, 0f32);
	brain.input[0].value = 1f32;
	brain.compute_neurons_state(&genes);
	assert_eq!(brain.input[0].value, 1f32);
	assert_gt!(brain.internal[0].value, 1f32 - EPSILON);
	assert_gt!(brain.output[0].value, 1f32 - EPSILON);
	assert_gt!(brain.output[1].value, 1f32 - EPSILON);
}

#[test]
fn should_compute_internal_connected_another_internal() {
	let mut brain = Brain::init(2);
	let genes = Vec::from([
		Gene::init(NeuronLayer::Input, 0, NeuronLayer::Internal, 0, 32767i16),
		Gene::init(NeuronLayer::Internal, 0, NeuronLayer::Internal, 1, 32767i16),
	]);
	assert_eq!(brain.output[0].value, 0f32);
	brain.input[0].value = 1f32;
	brain.compute_neurons_state(&genes);
	assert_eq!(brain.input[0].value, 1f32);
	assert_gt!(brain.internal[0].value, 1f32 - EPSILON);
	assert_gt!(brain.internal[1].value, 1f32 - EPSILON);
}

///
/// Input neurons
///

#[test]
fn should_set_block_forward_true() {
	let mut neuron = Neuron {
		neuron_type: NeuronType::BlockForward,
		neuron_layer: NeuronLayer::Input,
		value: 0f32,
	};
	let mut world = world::World::init();
	let position = world::Position { x: 1, y: 1 };
	let direction = world::Direction::North;
	let boundary = world::Size {
		height: 128,
		width: 128,
	};

	assert_eq!(neuron.value, 0f32);

	// one creature blocking the path forward
	world
		.coordinates
		.insert(world::Position { x: 1, y: 2 }, Creature::init(0, 0));
	neuron.set_from_world(&world, &position, &direction);
	assert_eq!(neuron.value, 1f32);
}

#[test]
fn should_set_block_forward_false() {
	let mut neuron = Neuron {
		neuron_type: NeuronType::BlockForward,
		neuron_layer: NeuronLayer::Input,
		value: 0f32,
	};
	let world = world::World::init();
	let position = world::Position { x: 1, y: 1 };
	let direction = world::Direction::North;

	assert_eq!(neuron.value, 0f32);

	// nothing blocking the path forward
	neuron.set_from_world(&world, &position, &direction);
	assert_eq!(neuron.value, 0f32);
}

#[test]
fn should_set_block_right_true() {
	let mut neuron = Neuron {
		neuron_type: NeuronType::BlockLeftRight,
		neuron_layer: NeuronLayer::Input,
		value: 0f32,
	};
	let mut world = world::World::init();
	let position = world::Position { x: 1, y: 1 };
	let direction = world::Direction::North;

	assert_eq!(neuron.value, 0f32);

	// one creature blocking the path left
	world
		.coordinates
		.insert(world::Position { x: 2, y: 1 }, Creature::init(0, 0));
	neuron.set_from_world(&world, &position, &direction);
	assert_eq!(neuron.value, 1f32);
}

#[test]
fn should_set_block_left_true() {
	let mut neuron = Neuron {
		neuron_type: NeuronType::BlockLeftRight,
		neuron_layer: NeuronLayer::Input,
		value: 0f32,
	};
	let mut world = world::World::init();
	let position = world::Position { x: 1, y: 1 };
	let direction = world::Direction::North;

	assert_eq!(neuron.value, 0f32);

	// one creature blocking the path left
	world
		.coordinates
		.insert(world::Position { x: 0, y: 1 }, Creature::init(0, 0));
	neuron.set_from_world(&world, &position, &direction);
	assert_eq!(neuron.value, 1f32);
}

#[test]
fn should_set_block_lateral_false() {
	let mut neuron = Neuron {
		neuron_type: NeuronType::BlockLeftRight,
		neuron_layer: NeuronLayer::Input,
		value: 0f32,
	};
	let world = world::World::init();
	let position = world::Position { x: 1, y: 1 };
	let direction = world::Direction::North;

	assert_eq!(neuron.value, 0f32);

	// nothing blocking the path laterally
	neuron.set_from_world(&world, &position, &direction);
	assert_eq!(neuron.value, 0f32);
}

///
/// Output neurons
///

#[test]
fn should_want_move_forward() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveForward,
		neuron_layer: NeuronLayer::Output,
		value: 1f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 0f32, y: 1f32 }
	);
}

#[test]
fn should_want_move_not_forward() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveForward,
		neuron_layer: NeuronLayer::Output,
		value: 0f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 0f32, y: 0f32 }
	);
}

#[test]
fn should_want_move_never_backward() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveForward,
		neuron_layer: NeuronLayer::Output,
		value: -1f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 0f32, y: 0f32 }
	);
}

#[test]
fn should_move_randomly() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveRandom,
		neuron_layer: NeuronLayer::Output,
		value: 1f32,
	};

	let delta = neuron.desired_move(&world::Direction::North);
	assert_le!(delta.x, 1f32);
	assert_ge!(delta.x, -1f32);
	assert_le!(delta.y, 1f32);
	assert_ge!(delta.y, -1f32);
}

#[test]
fn should_want_move_reverse() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveReverse,
		neuron_layer: NeuronLayer::Output,
		value: 1f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 0f32, y: -1f32 }
	);
}

#[test]
fn should_want_move_reverse_never_forward() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveReverse,
		neuron_layer: NeuronLayer::Output,
		value: -1f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 0f32, y: 0f32 }
	);
}

#[test]
fn should_want_move_right() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveLeftRight,
		neuron_layer: NeuronLayer::Output,
		value: 1f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 1f32, y: 0f32 }
	);
}

#[test]
fn should_want_move_left() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveLeftRight,
		neuron_layer: NeuronLayer::Output,
		value: -1f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: -1f32, y: 0f32 }
	);
}

#[test]
fn should_want_to_not_move_laterally() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveLeftRight,
		neuron_layer: NeuronLayer::Output,
		value: 0f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 0f32, y: 0f32 }
	);
}

#[test]
fn should_want_to_move_east() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveEastWest,
		neuron_layer: NeuronLayer::Output,
		value: 1f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 1f32, y: 0f32 }
	);
}

#[test]
fn should_want_to_move_west() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveEastWest,
		neuron_layer: NeuronLayer::Output,
		value: -1f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: -1f32, y: 0f32 }
	);
}

#[test]
fn should_want_to_not_move_east_west() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveEastWest,
		neuron_layer: NeuronLayer::Output,
		value: 0f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 0f32, y: 0f32 }
	);
}

#[test]
fn should_want_to_move_north() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveNorthSouth,
		neuron_layer: NeuronLayer::Output,
		value: 1f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 0f32, y: 1f32 }
	);
}

#[test]
fn should_want_to_move_south() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveNorthSouth,
		neuron_layer: NeuronLayer::Output,
		value: -1f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 0f32, y: -1f32 }
	);
}

#[test]
fn should_want_to_not_move_north_south() {
	let neuron = Neuron {
		neuron_type: NeuronType::MoveNorthSouth,
		neuron_layer: NeuronLayer::Output,
		value: 0f32,
	};

	assert_eq!(
		neuron.desired_move(&world::Direction::North),
		world::DeltaPosition { x: 0f32, y: 0f32 }
	);
}
