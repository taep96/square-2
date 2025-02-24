use macroquad::prelude::*;
use square_2::scenes::SceneManager;

#[macroquad::main("Square 2")]
async fn main() {
	let mut scene_manager = SceneManager::new();

	while !scene_manager.update() {
		scene_manager.render();
		next_frame().await
	}
}
