use crate::snake::Snake;
use crate::snake::Vector2;
use crate::snake::Direction;

use crate::scene::Scene;

use core::time::Duration;
use std::time::Instant;

// Render constants
const BACKGROUND_OUTER_COLOR: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 0, 0);
const BACKGROUND_INNER_COLOR: sdl2::pixels::Color = sdl2::pixels::Color::RGB(86, 74, 74);
const SNAKE_BODY_COLOR: sdl2::pixels::Color = sdl2::pixels::Color::RGB(159, 230, 160);
const SNAKE_HEAD_COLOR: sdl2::pixels::Color = sdl2::pixels::Color::RGB(74, 169, 108);
const APPLE_COLOR: sdl2::pixels::Color = sdl2::pixels::Color::RGB(245, 92, 71);

pub struct SnakeScene {
	// Configuration
	tick_length: Duration,
	board_size: Vector2,
	apple_count: usize,

	// High level state
	game: Snake,
	input_buffer: Vec<Direction>, // This is reversed from what you might think it is -- the latest input is at the end and gets popped, unlike my TS snake where the latest input is at the start and gets shifted. This is because Rust doesn't have a O(1) shift method like TS does, but it does have O(1) pop.

	// Low level state
	last_tick: Instant
} impl SnakeScene {
	pub fn new(tick_length: Duration, board_size: Vector2, apple_count: usize) -> Self {
		SnakeScene {
			tick_length,
			board_size,
			apple_count,

			game: Snake::new(board_size, apple_count),
			input_buffer: Vec::new(),

			last_tick: Instant::now()
		}
	}
} impl Scene for SnakeScene {
	fn event(&mut self, event: &sdl2::event::Event) -> () {
		match event {
			// State controls
			sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), .. } => {
				self.game = Snake::new(self.board_size, self.apple_count);
			},

			// Snake controls
			sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => {
				self.input_buffer.insert(0, Direction::NegativeX);
			},
			sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => {
				self.input_buffer.insert(0, Direction::PositiveX);
			},
			sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => {
				self.input_buffer.insert(0, Direction::NegativeY);
			},
			sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => {
				self.input_buffer.insert(0, Direction::PositiveY);
			},

			_ => {}
		}
	}

	fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
		// Check if we need to tick the snake (and do so)
		if self.last_tick.elapsed() > self.tick_length {
			self.game.tick(self.input_buffer.pop());
			self.last_tick = Instant::now();
		}

		// Clear canvas with black
		canvas.set_draw_color(BACKGROUND_OUTER_COLOR);
		canvas.clear();

		// Calculate tile size
		let (window_size_x, window_size_y) = canvas.output_size().unwrap();
		let tile_size = core::cmp::min(window_size_x / self.game.board_size.x as u32, window_size_y / self.game.board_size.y as u32);
		let offset = Vector2 {
			x: ((window_size_x - (tile_size * self.game.board_size.x as u32)) / 2) as i32,
			y: ((window_size_y - (tile_size * self.game.board_size.y as u32)) / 2) as i32
		};

		// Draw background
		canvas.set_draw_color(BACKGROUND_INNER_COLOR);
		let _ = canvas.fill_rect(
			sdl2::rect::Rect::new(offset.x, offset.y, tile_size * self.game.board_size.x as u32, tile_size * self.game.board_size.y as u32)
		);

		// Draw snake
		canvas.set_draw_color(SNAKE_BODY_COLOR);
		for &snake_piece in &self.game.snake[1..] {
			let _ = canvas.fill_rect(
				sdl2::rect::Rect::new((tile_size as i32 * snake_piece.x) + offset.x, (tile_size as i32 * snake_piece.y) + offset.y as i32, tile_size, tile_size)
			); 
		}

		// Snake head (different color)
		canvas.set_draw_color(SNAKE_HEAD_COLOR);
		let _ = canvas.fill_rect(
			sdl2::rect::Rect::new((tile_size as i32 * self.game.snake[0].x) + offset.x, (tile_size as i32 * self.game.snake[0].y) + offset.y as i32, tile_size, tile_size)
		); 

		// Draw apples
		canvas.set_draw_color(APPLE_COLOR);
		for &apple in &self.game.apples {
			let _ = canvas.fill_rect(
				sdl2::rect::Rect::new((tile_size as i32 * apple.x) + offset.x, (tile_size as i32 * apple.y) + offset.y, tile_size, tile_size)
			); 
		}

		canvas.present();
	}
}