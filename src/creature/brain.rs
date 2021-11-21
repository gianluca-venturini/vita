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
	fn init(description: BrainDescription) -> Brain {
		Brain {
			input: INPUT_NEURONS
				.to_vec()
				.into_iter()
				.map(|neuron_type| Neuron { neuron_type })
				.collect(),
			output: OUTPUT_NEURONS
				.to_vec()
				.into_iter()
				.map(|neuron_type| Neuron { neuron_type })
				.collect(),
			internal: vec![
				Neuron {
					neuron_type: NeuronType::Internal
				};
				description.num_internal as usize
			],
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
}
