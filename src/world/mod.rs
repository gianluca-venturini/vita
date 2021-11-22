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
	// Note that the world contains a copy of the creatures, not a reference to them.
	// The function update_creatures_positions() should be called every time that the position change.
	pub coordinates: HashMap<Position, creature::Creature>,
	pub boundary: Size,
}

impl World {
	pub fn init() -> World {
		World {
			coordinates: HashMap::new(),
			boundary: Size {
				height: 128,
				width: 128,
			},
		}
	}

	// This function resets the world in a state coherent with the creature position.
	pub fn update_creatures_positions(&mut self, creatures: &Vec<creature::Creature>) {
		self.coordinates.clear();
		for creature in creatures.iter() {
			self.coordinates.insert(creature.position, creature.clone());
		}
	}

	// This function encodes all the complexity of the physics in the world::World.
	// This function returns the next position that will be assumed by the entity.
	// The world needs to know already that some entity is in that position, otherwise will panic.
	pub fn move_creature(
		&mut self,
		current_position: &Position,
		delta: &DeltaPosition,
	) -> Position {
		let creature = self.coordinates.get(current_position);
		if creature.is_none() {
			println!("No entity found in world position {:?}", current_position);
			panic!("Position not found");
		}
		// creature.unwrap()
		// next_position = current_position.move_delta(delta)
		*current_position
	}
}

#[derive(Debug)]
pub struct Size {
	pub width: u16,
	pub height: u16,
}

#[derive(Debug, std::hash::Hash, PartialEq, std::cmp::Eq, Clone, Copy)]
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

	pub fn move_delta(&self, delta: &DeltaPosition, max_step: u16) -> Position {
		let x = (self.x as f32 + delta.x.clamp(-(max_step as f32), max_step as f32)).floor();
		let y = (self.y as f32 + delta.y.clamp(-(max_step as f32), max_step as f32)).floor();
		Position {
			x: if x > 0f32 { x as u16 } else { 0 },
			y: if y > 0f32 { y as u16 } else { 0 },
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

#[derive(Debug, Clone, Copy)]
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

#[test]
fn should_move_position_delta() {
	assert_eq!(
		Position { x: 0u16, y: 0u16 }.move_delta(&DeltaPosition { x: 1f32, y: 0f32 }, 1),
		Position { x: 1u16, y: 0u16 }
	);
}
