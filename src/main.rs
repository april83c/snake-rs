// This will make is to that a "release" build (with the --release flag) will use the "windows" subsystem on Windows, instead of the "console" subsystem. This makes the process not have a console by default, which prevents a little terminal window from running in the background when the program runs on its own. However, we only want that in release mode because we want the ability to print debug message in debug mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod snake;
use snake::Snake;
use snake::SnakeState;
use snake::Vector2;
use snake::Direction;

use std::panic;
use std::ptr::null;
use panic_message::panic_info_message;

use std::cmp;

use std::time::{Duration, Instant};

fn main() {
	// Panic hook (show errors)
	panic::set_hook(Box::new(|panic_info| {
		match sdl2::messagebox::show_simple_message_box(sdl2::messagebox::MessageBoxFlag::ERROR, "Panic", panic_info_message(panic_info), None) {
			Ok(result) => result,
			Err(message_box_error) => {
				eprintln!("Panic: {}", panic_info_message(panic_info));
				eprintln!("Additionally, an error occured showing a message box with the panic: {}", message_box_error);
			}
		}
	}));

	// Set up SDL context and canvas
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem
		.window("Snake", 640, 480)
		.resizable()
		.position_centered()
		.build()
		.unwrap();
	let mut canvas = window.into_canvas().present_vsync().build().unwrap(); // present_vsync enables V-Sync in some way, but i'm not entirely sure how it works? does it just make the present() call sleep for a bit? idk.
	let mut event_pump = sdl_context.event_pump().unwrap();

	// Set up snake
	let mut snake = Snake::new(Vector2 { x: 10, y: 10 }, 1);

	// Main loop
	const TICK_LENGTH: Duration = Duration::from_millis(100);
	let mut last_tick = Instant::now() - TICK_LENGTH;

	let mut input_buffer: Vec<Direction> = Vec::new(); // This is reversed from what you might think it is -- the latest input is at the end and gets popped, unlike my TS snake where the latest input is at the start and gets shifted. This is because Rust doesn't have a O(1) shift method like TS does, but it does have O(1) pop.
	'main: loop {
		// Process events
		for event in event_pump.poll_iter() {
			match event {
				sdl2::event::Event::Quit {..} => {
					break 'main;
				},
				sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => {
					input_buffer.insert(0, Direction::NegativeX);
				},
				sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => {
					input_buffer.insert(0, Direction::PositiveX);
				},
				sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => {
					input_buffer.insert(0, Direction::NegativeY);
				},
				sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => {
					input_buffer.insert(0, Direction::PositiveY);
				},
				_ => {}
			}
		}

		// Check if we need to tick the snake (and do so)
		if last_tick.elapsed() > TICK_LENGTH {
			snake.tick(input_buffer.pop());
			last_tick = Instant::now();
		}

		// Clear canvas with black
		canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
		canvas.clear();

		// Calculate tile size
		let (window_size_x, window_size_y) = canvas.output_size().unwrap();
		let tile_size = cmp::min(window_size_x / snake.board_size.x as u32, window_size_y / snake.board_size.y as u32);
		let offset = Vector2 {
			x: ((window_size_x - (tile_size * snake.board_size.x as u32)) / 2) as i32,
			y: ((window_size_y - (tile_size * snake.board_size.y as u32)) / 2) as i32
		};

		// Draw background
		canvas.set_draw_color(sdl2::pixels::Color::GRAY);
		let _ = canvas.fill_rect(
			sdl2::rect::Rect::new(offset.x, offset.y, tile_size * snake.board_size.x as u32, tile_size * snake.board_size.y as u32)
		);

		// Draw snake
		canvas.set_draw_color(sdl2::pixels::Color::GREEN);
		for &snake_piece in &snake.snake {
			let _ = canvas.fill_rect(
				sdl2::rect::Rect::new((tile_size as i32 * snake_piece.x) + offset.x, (tile_size as i32 * snake_piece.y) + offset.y as i32, tile_size, tile_size)
			); // we don't care about errors here i guess?
		}

		// Draw apples
		canvas.set_draw_color(sdl2::pixels::Color::RED);
		for &apple in &snake.apples {
			let _ = canvas.fill_rect(
				sdl2::rect::Rect::new((tile_size as i32 * apple.x) + offset.x, (tile_size as i32 * apple.y) + offset.y, tile_size, tile_size)
			); // we don't care about errors here i guess?
		}

		canvas.present();
	}
}