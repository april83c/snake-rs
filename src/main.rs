use rand::Rng;
use core::ops::Add;
use std::{io::Read, thread, time}; // for rendering, remove this later?

fn main() {
	let mut snake = Snake::new(Vector2 { x: 10, y: 10 }, 1);
	let duration = time::Duration::from_millis(150);

	loop {
		snake.tick(None);
		render(&snake);
		thread::sleep(duration);
	}
}

fn render(snake: &Snake) {
	// position cursor at row 1 col 1 of terminal
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

	for row in 0..snake.board_size.y {
		for column in 0..snake.board_size.x {
			let position = Vector2 { x: column, y: row };
			if let Some(snake_index) = snake.snake.iter().position(|snake_piece| snake_piece == &position) {
				if snake_index == 0 {
					print!("🐍");
				} else {
					print!("🟩");
				}
			} else if snake.apples.contains(&position) {
				print!("🍎");
			} else {
				print!("⬛");
			}
		}
		print!("\n");
	}
	if snake.state != SnakeState::Running {
		println!("Game over");
	}
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Vector2 {
	x: i32,
	y: i32
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
	NegativeX, PositiveX,
	NegativeY, PositiveY
}

impl Direction {
	fn flip(self) -> Self {
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
enum SnakeState {
	Running,
	DeadByBody,
	DeadByOutOfBounds
}

#[derive(Debug)]
struct Snake {
	board_size: Vector2,

	snake: Vec<Vector2>,
	apples: Vec<Vector2>,

	state: SnakeState,
	snake_velocity: Direction
}
impl Snake {
	fn new(board_size: Vector2, apple_count: usize) -> Self {
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

	fn tick(&mut self, input: Option<Direction>) -> bool {
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