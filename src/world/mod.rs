use super::creature;
use std::collections::HashMap;

// The world coordinate system has (0, 0) on bottom left

pub struct World {
	pub coordinates: HashMap<Position, creature::Creature>,
	pub creature: Vec<creature::Creature>,
}

impl World {
	pub fn init() -> World {
		World {
			coordinates: HashMap::new(),
			creature: Vec::new(),
		}
	}
}

#[derive(Debug, std::hash::Hash, PartialEq, std::cmp::Eq)]
pub struct Position {
	pub x: u16,
	pub y: u16,
}

impl Position {
	pub fn move_direction(&self, direction: &Direction, step: u16) -> Position {
		match direction {
			Direction::North => Position {
				x: self.x,
				y: self.y + step,
			},
			Direction::South => Position {
				x: self.x,
				y: self.y - step,
			},
			Direction::East => Position {
				x: self.x + step,
				y: self.y,
			},
			Direction::West => Position {
				x: self.x - step,
				y: self.y,
			},
		}
	}
}

#[derive(Debug)]
pub enum Direction {
	North,
	South,
	East,
	West,
}

#[test]
fn should_move_correctly() {
	assert_eq!(
		Position { x: 1, y: 1 }.move_direction(&Direction::North, 1),
		Position { x: 1, y: 2 }
	);
	assert_eq!(
		Position { x: 1, y: 1 }.move_direction(&Direction::South, 1),
		Position { x: 1, y: 0 }
	);
	assert_eq!(
		Position { x: 1, y: 1 }.move_direction(&Direction::East, 1),
		Position { x: 2, y: 1 }
	);
	assert_eq!(
		Position { x: 1, y: 1 }.move_direction(&Direction::West, 1),
		Position { x: 0, y: 1 }
	);
	assert_eq!(
		Position { x: 1, y: 1 }.move_direction(&Direction::North, 2),
		Position { x: 1, y: 3 }
	);
}
