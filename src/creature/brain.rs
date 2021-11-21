pub struct Brain {
	pub num_input: u8,
	pub num_internal: u8,
	pub num_output: u8,
}

#[derive(Debug, PartialEq)]
pub enum NeuronType {
	Input,
	Internal,
	Output,
}

#[derive(Debug, PartialEq)]
pub struct Neuron {
	pub neuron_type: NeuronType,
	pub neuron_number: u8,
}
