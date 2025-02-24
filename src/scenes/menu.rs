use crate::ui::button::Button;
use macroquad::prelude::*;

use super::{Scene, SceneBehavior, Transition};

pub struct Menu {
	buttons: Vec<Button>,
}

impl Menu {
	pub fn new() -> Self {
		let play_button = Button::menu_button("PLAY", 40.0, || Some(Transition::ToGame));
		let quit_button = Button::menu_button("QUIT", 52.0, || Some(Transition::Quit));

		Menu {
			buttons: vec![play_button, quit_button],
		}
	}
}

// Menu overrides the default escape behavior since it's the top-level scene
impl SceneBehavior for Menu {
	fn handle_common_input(&self) -> Option<Transition> {
		if is_key_pressed(KeyCode::Escape) {
			Some(Transition::Quit)
		} else {
			None
		}
	}
}

impl Scene for Menu {
	fn update(&mut self) -> Option<Transition> {
		// Check common behavior first (like escape key)
		if let Some(transition) = self.handle_common_input() {
			return Some(transition);
		}

		// Then check buttons
		for button in self.buttons.iter_mut() {
			if let Some(transition) = button.update() {
				return Some(transition);
			}
		}
		None
	}

	fn render(&mut self) {
		clear_background(BLACK);
		for button in &mut self.buttons {
			button.render();
		}
	}
}
