use game::scenes::SceneManager;
use macroquad::prelude::*;

#[macroquad::main("Square 2")]
async fn main() {
	let mut scene_manager = SceneManager::new();

	while !scene_manager.update() {
		scene_manager.render();
		next_frame().await
	}
}
