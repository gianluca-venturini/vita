use super::creature;
use std::collections::HashMap;

// The world coordinate system has (0, 0) on bottom left

pub struct World {
	coordinates: HashMap<Position, creature::Creature>,
	creature: Vec<creature::Creature>,
}

impl World {
	pub fn init() -> World {
		World {
			coordinates: HashMap::new(),
			creature: Vec::new(),
		}
	}
}

#[derive(Debug, std::hash::Hash)]
pub struct Position {
	pub x: u16,
	pub y: u16,
}

#[derive(Debug)]
pub enum Direction {
	North,
	South,
	East,
	West,
}
