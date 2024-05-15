pub trait Scene {
	fn event(&mut self, event: &sdl2::event::Event) -> ();
	fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> ();
}