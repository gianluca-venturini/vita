use super::brain::{BrainDescription, NeuronDescription, NeuronLayer};
use std::fmt::{self, Debug, Display, Formatter};

pub struct Gene {
	// source neuron
	source: u8,
	// destination neuron
	destination: u8,
	pub weight: i16,
}

impl Debug for Gene {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{} {} {}", self.source, self.destination, self.weight)
	}
}

impl Display for Gene {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"{}{}{}",
			&format!("{:#04X}", self.source)[2..],
			&format!("{:#04X}", self.destination)[2..],
			&format!("{:#06X}", self.weight as u16)[2..],
		)
	}
}

impl Gene {
	pub fn init(source: u8, destination: u8, weight: i16) -> Gene {
		return Gene {
			source,
			destination,
			weight,
		};
	}

	pub fn get_source_neuron_layer(&self) -> NeuronLayer {
		if self.source & 0b10000000 == 0 {
			return NeuronLayer::Input;
		} else {
			return NeuronLayer::Internal;
		}
	}

	pub fn get_destination_neuron_layer(&self) -> NeuronLayer {
		println!("{}", self.destination & 0b10000000);
		if self.destination & 0b10000000 == 0 {
			return NeuronLayer::Internal;
		} else {
			return NeuronLayer::Output;
		}
	}

	pub fn get_source_neuron(&self, brain: &BrainDescription) -> NeuronDescription {
		let neuron_layer = self.get_source_neuron_layer();
		Gene::get_neuron(neuron_layer, self.source, brain)
	}

	pub fn get_destination_neuron(&self, brain: &BrainDescription) -> NeuronDescription {
		let neuron_layer = self.get_destination_neuron_layer();
		Gene::get_neuron(neuron_layer, self.destination, brain)
	}

	pub fn mutate(&mut self, bit: u8) {
		if bit >= 32 {
			panic!()
		}
		let raw_gene = u32::from(self.source) << (16 + 8)
			| u32::from(self.destination) << 16
			| (self.weight as u32);
		let new_raw_gene = raw_gene ^ (0b1 << bit);
		self.source = (new_raw_gene >> (16 + 8)) as u8;
		self.destination = (new_raw_gene >> (16)) as u8;
		self.weight = new_raw_gene as i16;
	}

	fn get_neuron(
		neuron_layer: NeuronLayer,
		raw_number: u8,
		brain: &BrainDescription,
	) -> NeuronDescription {
		let neuron_number = match neuron_layer {
			NeuronLayer::Input => (raw_number & 0b01111111) % brain.num_input,
			NeuronLayer::Internal => (raw_number & 0b01111111) % brain.num_input,
			NeuronLayer::Output => (raw_number & 0b01111111) % brain.num_output,
		};
		NeuronDescription {
			neuron_layer,
			neuron_number,
		}
	}
}

#[test]
fn should_select_source_type() {
	assert_eq!(
		Gene::init(0, 0, 0).get_source_neuron_layer(),
		NeuronLayer::Input
	);
	assert_eq!(
		Gene::init(128, 0, 0).get_source_neuron_layer(),
		NeuronLayer::Internal
	);
}

#[test]
fn should_select_destination_type() {
	assert_eq!(
		Gene::init(0, 0, 0).get_destination_neuron_layer(),
		NeuronLayer::Internal
	);
	assert_eq!(
		Gene::init(0, 128, 0).get_destination_neuron_layer(),
		NeuronLayer::Output
	);
}

#[test]
fn should_select_source_neuron() {
	let brain = BrainDescription {
		num_input: 5,
		num_output: 5,
		num_internal: 5,
	};
	assert_eq!(
		Gene::init(0, 0, 0).get_source_neuron(&brain),
		NeuronDescription {
			neuron_layer: NeuronLayer::Input,
			neuron_number: 0
		}
	);
	assert_eq!(
		Gene::init(1, 0, 0).get_source_neuron(&brain),
		NeuronDescription {
			neuron_layer: NeuronLayer::Input,
			neuron_number: 1
		}
	);
	assert_eq!(
		Gene::init(5, 0, 0).get_source_neuron(&brain),
		NeuronDescription {
			neuron_layer: NeuronLayer::Input,
			neuron_number: 0
		}
	);
	assert_eq!(
		Gene::init(128, 0, 0).get_source_neuron(&brain),
		NeuronDescription {
			neuron_layer: NeuronLayer::Internal,
			neuron_number: 0
		}
	);
	assert_eq!(
		Gene::init(128 + 1, 0, 0).get_source_neuron(&brain),
		NeuronDescription {
			neuron_layer: NeuronLayer::Internal,
			neuron_number: 1
		}
	);
	assert_eq!(
		Gene::init(128 + 5, 0, 0).get_source_neuron(&brain),
		NeuronDescription {
			neuron_layer: NeuronLayer::Internal,
			neuron_number: 0
		}
	);
}

#[test]
fn should_display_correctly() {
	assert_eq!(format!("{}", Gene::init(0, 0, 0)), "00000000");
	assert_eq!(format!("{}", Gene::init(255, 255, -1)), "FFFFFFFF");
	assert_eq!(format!("{}", Gene::init(255, 0, -1)), "FF00FFFF");
	assert_eq!(format!("{}", Gene::init(0, 255, -1)), "00FFFFFF");
	assert_eq!(format!("{}", Gene::init(255, 255, 0)), "FFFF0000");
}

#[test]
fn should_mutate() {
	fn init_and_mutate(bit: u8) -> Gene {
		let mut gene = Gene::init(0, 0, 0);
		gene.mutate(bit);
		gene
	}
	assert_eq!(format!("{}", init_and_mutate(0)), "00000001");
	assert_eq!(format!("{}", init_and_mutate(1)), "00000002");
	assert_eq!(format!("{}", init_and_mutate(2)), "00000004");
	assert_eq!(format!("{}", init_and_mutate(3)), "00000008");
	assert_eq!(format!("{}", init_and_mutate(4)), "00000010");
	assert_eq!(format!("{}", init_and_mutate(31)), "80000000");
}
