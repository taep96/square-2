mod game;
mod menu;

pub use self::game::Game;
pub use self::menu::Menu;

use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Transition {
	/// Switch to the menu scene
	ToMenu,
	/// Switch to the game scene
	ToGame,
	/// Exit the game
	Quit,
}

/// Common behavior for all scenes
pub trait SceneBehavior {
	/// Handle common input like escape key
	fn handle_common_input(&self) -> Option<Transition> {
		if is_key_pressed(KeyCode::Escape) {
			Some(Transition::ToMenu)
		} else {
			None
		}
	}
}

/// A scene in the game (menu, game, etc.)
pub trait Scene: SceneBehavior {
	/// Update the scene state
	/// Returns Some(Transition) if the scene wants to transition, None to continue
	fn update(&mut self) -> Option<Transition> {
		self.handle_common_input()
	}

	/// Render the scene
	fn render(&mut self);
}

/// Manages the current scene and handles transitions between scenes
pub struct SceneManager {
	current_scene: Box<dyn Scene>,
}

impl SceneManager {
	pub fn new() -> Self {
		Self {
			current_scene: Box::new(Menu::new()),
		}
	}

	/// Process a scene transition
	pub fn transition(&mut self, transition: Transition) -> bool {
		match transition {
			Transition::Quit => true,
			Transition::ToMenu => {
				self.current_scene = Box::new(Menu::new());
				false
			}
			Transition::ToGame => {
				self.current_scene = Box::new(Game::new());
				false
			}
		}
	}

	/// Update the current scene
	/// Returns true when it's time to quit, false otherwise
	pub fn update(&mut self) -> bool {
		if let Some(transition) = self.current_scene.update() {
			self.transition(transition)
		} else {
			false
		}
	}

	pub fn render(&mut self) {
		self.current_scene.render();
	}
}
