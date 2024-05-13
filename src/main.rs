mod snake;
use snake::Snake;
use snake::SnakeState;
use snake::Vector2;

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