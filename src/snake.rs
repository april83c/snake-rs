use rand::Rng;
use core::ops::Add;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector2 {
	pub x: i32,
	pub y: i32
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
	NegativeX, PositiveX,
	NegativeY, PositiveY
}

impl Direction {
	pub fn flip(self) -> Self {
		match self {
			Direction::NegativeX => return Direction::PositiveX,
			Direction::PositiveX => return Direction::NegativeX,
			Direction::NegativeY => return Direction::PositiveY,
			Direction::PositiveY => return Direction::NegativeY
		}
	}
}

impl Add<Direction> for Vector2 {
	type Output = Vector2;

	fn add(mut self, other: Direction) -> Vector2 {
		match other {
			Direction::NegativeX => self.x -= 1,
			Direction::PositiveX => self.x += 1,
			Direction::NegativeY => self.y -= 1,
			Direction::PositiveY => self.y += 1
		}
		self
	}
}

#[derive(Debug, PartialEq)]
pub enum SnakeState {
	Running,
	DeadByBody,
	DeadByOutOfBounds
}

#[derive(Debug)]
pub struct Snake {
	pub board_size: Vector2,

	pub snake: Vec<Vector2>,
	pub apples: Vec<Vector2>,

	pub state: SnakeState,
	pub snake_velocity: Direction
}
impl Snake {
	pub fn new(board_size: Vector2, apple_count: usize) -> Self {
		let mut new = Self {
			board_size,

			snake: Vec::new(),
			apples: Vec::new(),

			state: SnakeState::Running,
			snake_velocity: Direction::PositiveX
		};

		new.snake.push(Vector2 {
			x: board_size.x / 2,
			y: board_size.y / 2
		});

		for _ in 0..apple_count {
			let pos_result = new.generate_valid_apple_position();
			match pos_result {
				Ok(pos) => {
					new.apples.push(pos);
				},
				Err(_) => {
					break;
				}
			}
		}

		return new;
	}

	fn generate_valid_apple_position(&self) -> Result<Vector2, bool> {
		let mut rng = rand::thread_rng();

		if self.board_size.x * self.board_size.y <= self.apples.len() as i32 {
			println!("meow?!");
			return Err(false);
		}

		loop {
			let pos = Vector2 { 
				x: rng.gen_range(0..self.board_size.x),
				y: rng.gen_range(0..self.board_size.y)
			};

			if !self.snake.contains(&pos) && !self.apples.contains(&pos) {
				return Ok(pos);
			} else {
				continue;
			}
		}
	}

	pub fn tick(&mut self, input: Option<Direction>) -> bool {
		if self.state != SnakeState::Running { return true };

		// validate input (check it exists, stop 180s) then set it as snake velocity
		if let Some(input) = input {
			if input != self.snake_velocity.flip() {
				self.snake_velocity = input;
			}
		}

		// snake movement
		let next_position = self.snake[0] + self.snake_velocity;

		// check if we are out of bounds
		if next_position.x >= self.board_size.x || next_position.x < 0
		|| next_position.y >= self.board_size.y || next_position.y < 0 {
			self.state = SnakeState::DeadByOutOfBounds;
			return true;
		} // check if we are colliding with ourselves
		else if self.snake.contains(&next_position) { 
			self.state = SnakeState::DeadByBody;
			// we want it to actually be reflected in the snake, so that
			// it looks like we actually did run into ourselves!
			self.snake.insert(0, next_position);
			return true;
		}

		// everything is okay, add new position
		self.snake.insert(0, next_position);

		// check if we are eating an apple
		if let Some(index) = self.apples.iter().position(|apple| apple == &next_position) {
			self.apples.remove(index);
		} else {
			self.snake.pop();
		}

		return true;
	}
}