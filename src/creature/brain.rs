use super::gene::Gene;
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

pub struct Brain {
	input: Vec<Neuron>,
	internal: Vec<Neuron>,
	output: Vec<Neuron>,
}

impl Brain {
	pub fn init(description: BrainDescription) -> Brain {
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
				description.num_internal as usize
			],
		}
	}

	pub fn compute_neurons_state(&mut self, genes: &Vec<Gene>) {
		// Reset all neurons
		// IDEA: maybe these neuron layer should be set outside and we can avoid touching it here
		self.reset_neurons_layer(NeuronLayer::Input);
		self.reset_neurons_layer(NeuronLayer::Internal);
		self.reset_neurons_layer(NeuronLayer::Output);
		let connections = self.get_connection_from_genes(genes);

		// Compute all neurons with input layer source
		self.compute_sum_on_destination_neurons(
			&connections,
			NeuronLayer::Input,
			NeuronLayer::Internal,
		);
		self.compute_sum_on_destination_neurons(
			&connections,
			NeuronLayer::Input,
			NeuronLayer::Output,
		);

		// Compute all internal neurons that are connected to intermediate neurons
		self.compute_sum_on_destination_neurons(
			&connections,
			NeuronLayer::Internal,
			NeuronLayer::Internal,
		);

		// Compute all neurons with intermediate layer source
		self.compute_sum_on_destination_neurons(
			&connections,
			NeuronLayer::Internal,
			NeuronLayer::Output,
		);
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

	fn compute_sum_on_destination_neurons(
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
				let mut weightedValue = 0f32;
				{
					let source = self.desc_to_neuron(&connection.source);
					weightedValue = source.value * connection.weight;
				}
				let destination = self.desc_to_neuron(&connection.destination);
				changes.insert(
					// accumulate the value
					connection.destination.neuron_number,
					match changes.get(&connection.destination.neuron_number) {
						Some(x) => *x,
						None => 0f32,
					} + weightedValue,
				);
			}
		}
		// Now is safe to apply the changes
		for (neuron_number, valueChange) in changes.iter_mut() {
			let neuron = self.desc_to_neuron(&NeuronDescription {
				neuron_number: *neuron_number,
				neuron_layer: destination_layer,
			});
			neuron.value = (neuron.value.atanh() + *valueChange).tanh();
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

#[derive(Clone, std::hash::Hash)]
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

#[derive(Clone)]
pub struct Neuron {
	neuron_type: NeuronType,
	neuron_layer: NeuronLayer,
	value: f32,
}

struct NeuronConnection {
	source: NeuronDescription,
	destination: NeuronDescription,
	weight: f32,
}
