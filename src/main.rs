// This will make is to that a "release" build (with the --release flag) will use the "windows" subsystem on Windows, instead of the "console" subsystem. This makes the process not have a console by default, which prevents a little terminal window from running in the background when the program runs on its own. However, we only want that in release mode because we want the ability to print debug message in debug mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod snake;
use snake::Snake;
use snake::Vector2;
use snake::Direction;

mod scene;
use scene::Scene;

use std::panic;
use panic_message::panic_info_message;

use std::cmp;

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

	// Set up scene
	let mut current_scene = scene::SnakeScene::new(
		core::time::Duration::from_millis(100),
		Vector2 { x: 10, y: 10 },
		5
	);

	'main: loop {
		// Process events
		for event in event_pump.poll_iter() {
			match event {
				sdl2::event::Event::Quit {..} => {
					break 'main;
				},

				_ => {
					current_scene.event(&event);
				}
			}
		}

		current_scene.draw(&mut canvas);	
	}
}