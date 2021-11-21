use super::gene::Gene;

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
				})
				.collect(),
			output: OUTPUT_NEURONS
				.to_vec()
				.into_iter()
				.map(|neuron_type| Neuron {
					neuron_type,
					value: 0f32,
				})
				.collect(),
			internal: vec![
				Neuron {
					neuron_type: NeuronType::Internal,
					value: 0f32
				};
				description.num_internal as usize
			],
		}
	}

	pub fn compute_neurons_state(&self, genes: &Vec<Gene>) {
		// TODO: reset all neurons
		let connection = self.get_connection_from_genes(genes);
		// TODO: compute all neurons with input layer source
		// TODO: compute all neurons with input layer intermediate
	}

	fn get_connection_from_genes<'a>(&'a self, genes: &Vec<Gene>) -> Vec<NeuronConnection<'a>> {
		let mut connections: Vec<NeuronConnection<'a>> = Vec::new();

		let desc_to_neuron = |desc: NeuronDescription| -> &'a Neuron {
			match desc.neuron_layer {
				NeuronLayer::Input => &self.input[desc.neuron_number as usize],
				NeuronLayer::Internal => &self.internal[desc.neuron_number as usize],
				NeuronLayer::Output => &self.output[desc.neuron_number as usize],
			}
		};

		for gene in genes {
			let source_desc = gene.get_source_neuron(&self.to_brain_description());
			let destination_desc = gene.get_destination_neuron(&self.to_brain_description());
			let source = desc_to_neuron(source_desc);
			let destination = desc_to_neuron(destination_desc);
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

#[derive(Debug, PartialEq)]
pub enum NeuronLayer {
	Input,
	Internal,
	Output,
}

#[derive(Clone)]
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
	value: f32,
}

struct NeuronConnection<'a> {
	source: &'a Neuron,
	destination: &'a Neuron,
	weight: f32,
}
