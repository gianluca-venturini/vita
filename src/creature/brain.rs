use super::gene::Gene;
use super::world;
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

	// Internal
	Internal,

	// Output
	BorderDistanceNorthSouth,
	BorderDistanceEastWest,
	WordLocationNorthSouth,
	WordLocationEastWest,
}

const INPUT_NEURONS: [NeuronType; 5] = [
	NeuronType::Random,
	NeuronType::BlockLeftRight,
	NeuronType::BlockForward,
	NeuronType::LastMovementY,
	NeuronType::LastMovementX,
];

const OUTPUT_NEURONS: [NeuronType; 4] = [
	NeuronType::BorderDistanceNorthSouth,
	NeuronType::BorderDistanceEastWest,
	NeuronType::WordLocationNorthSouth,
	NeuronType::WordLocationEastWest,
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
			Random => {}
			BlockLeftRight => {}
			BlockForward => {}
			LastMovementY => {}
			LastMovementX => {}

			Internal => {}
			BorderDistanceNorthSouth => {}
			BorderDistanceEastWest => {}
			WordLocationNorthSouth => {}
			WordLocationEastWest => {}
		};
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

#[test]
fn should_set_input_neurons() {
	let mut neuron = Neuron {
		neuron_type: NeuronType::BlockForward,
		neuron_layer: NeuronLayer::Internal,
		value: 0f32,
	};
	let world = world::World::init();
	let position = world::Position { x: 1, y: 1 };
	let direction = world::Direction::North;
	neuron.set_from_world(&world, &position, &direction);
	assert_eq!(neuron.value, 1f32);
}
