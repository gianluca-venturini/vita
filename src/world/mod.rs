use super::creature;
use std::collections::HashMap;

// The world coordinate system has (0, 0) on bottom left
//
//  ^ Y
//  |
//  |
//  |
//  |
//  |
//  |
//  |
//  |
//  |
//  |
//  |
// -|-------------------------------------------> X
// (0,0)

pub struct World {
	pub coordinates: HashMap<Position, creature::Creature>,
	pub creature: Vec<creature::Creature>,
	pub boundary: Size,
}

impl World {
	pub fn init() -> World {
		World {
			coordinates: HashMap::new(),
			creature: Vec::new(),
			boundary: Size {
				height: 128,
				width: 128,
			},
		}
	}
}

#[derive(Debug)]
pub struct Size {
	pub width: u16,
	pub height: u16,
}

#[derive(Debug, std::hash::Hash, PartialEq, std::cmp::Eq)]
pub struct Position {
	pub x: u16,
	pub y: u16,
}

impl Position {
	pub fn move_direction(
		&self,
		direction: &Direction,
		step: u16,
		boundary: &Size,
	) -> Option<Position> {
		match direction {
			Direction::North => {
				if self.y + step >= boundary.height {
					return None;
				}
				Option::Some(Position {
					x: self.x,
					y: self.y + step,
				})
			}
			Direction::South => {
				if (self.y as i16 - step as i16) < 0 {
					return None;
				}
				Option::Some(Position {
					x: self.x,
					y: self.y - step,
				})
			}
			Direction::East => {
				if self.x + step >= boundary.width {
					return None;
				}
				Option::Some(Position {
					x: self.x + step,
					y: self.y,
				})
			}
			Direction::West => {
				if (self.x as i16 - step as i16) < 0 {
					return None;
				}
				Option::Some(Position {
					x: self.x - step,
					y: self.y,
				})
			}
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct DeltaPosition {
	pub x: f32,
	pub y: f32,
}

impl DeltaPosition {
	pub fn move_direction(&self, direction: &Direction, step: f32) -> DeltaPosition {
		match direction {
			Direction::North => DeltaPosition {
				x: self.x,
				y: self.y + step,
			},
			Direction::South => DeltaPosition {
				x: self.x,
				y: self.y - step,
			},
			Direction::East => DeltaPosition {
				x: self.x + step,
				y: self.y,
			},
			Direction::West => DeltaPosition {
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

impl Direction {
	pub fn rotate_left(&self) -> Direction {
		match self {
			Direction::North => Direction::West,
			Direction::East => Direction::North,
			Direction::South => Direction::East,
			Direction::West => Direction::South,
		}
	}

	pub fn rotate_right(&self) -> Direction {
		self.rotate_left().rotate_left().rotate_left()
	}
}

#[test]
fn should_move_correctly() {
	let boundary = Size {
		width: 128,
		height: 128,
	};
	assert_eq!(
		Position { x: 1, y: 1 }.move_direction(&Direction::North, 1, &boundary),
		Some(Position { x: 1, y: 2 })
	);
	assert_eq!(
		Position { x: 1, y: 1 }.move_direction(&Direction::South, 1, &boundary),
		Some(Position { x: 1, y: 0 })
	);
	assert_eq!(
		Position { x: 1, y: 1 }.move_direction(&Direction::East, 1, &boundary),
		Some(Position { x: 2, y: 1 })
	);
	assert_eq!(
		Position { x: 1, y: 1 }.move_direction(&Direction::West, 1, &boundary),
		Some(Position { x: 0, y: 1 })
	);
	assert_eq!(
		Position { x: 1, y: 1 }.move_direction(&Direction::North, 2, &boundary),
		Some(Position { x: 1, y: 3 })
	);
}

#[test]
fn should_move_only_inside_boundary() {
	let boundary = Size {
		width: 128,
		height: 128,
	};
	assert_eq!(
		Position { x: 127, y: 127 }.move_direction(&Direction::North, 1, &boundary),
		None
	);
	assert_eq!(
		Position { x: 0, y: 0 }.move_direction(&Direction::South, 1, &boundary),
		None
	);
	assert_eq!(
		Position { x: 127, y: 127 }.move_direction(&Direction::East, 1, &boundary),
		None
	);
	assert_eq!(
		Position { x: 0, y: 0 }.move_direction(&Direction::West, 1, &boundary),
		None
	);
}

#[test]
fn should_delta_move_correctly() {
	assert_eq!(
		DeltaPosition { x: 0f32, y: 0f32 }.move_direction(&Direction::North, 1f32),
		DeltaPosition { x: 0f32, y: 1f32 }
	);
	assert_eq!(
		DeltaPosition { x: 0f32, y: 0f32 }.move_direction(&Direction::South, 1f32),
		DeltaPosition { x: 0f32, y: -1f32 }
	);
	assert_eq!(
		DeltaPosition { x: 0f32, y: 0f32 }.move_direction(&Direction::East, 1f32),
		DeltaPosition { x: 1f32, y: 0f32 }
	);
	assert_eq!(
		DeltaPosition { x: 0f32, y: 0f32 }.move_direction(&Direction::West, 1f32),
		DeltaPosition { x: -1f32, y: 0f32 }
	);
}
