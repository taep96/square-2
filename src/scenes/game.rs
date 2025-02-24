use macroquad::prelude::*;

use crate::entities::*;

use super::{Scene, SceneBehavior, Transition};

pub struct Game {
	player1: Player,
	player2: Player,
	bullets: Vec<Bullet>,
}

const PLAYER1_SHOOT: KeyCode = KeyCode::E;
const PLAYER2_SHOOT: KeyCode = KeyCode::RightControl;

impl Game {
	pub fn new() -> Self {
		Self {
			player1: Player::new(PlayerColor::Red),
			player2: Player::new(PlayerColor::Blue),
			bullets: vec![],
		}
	}
}

impl SceneBehavior for Game {}

impl Scene for Game {
	fn update(&mut self) -> Option<Transition> {
		// Check common behavior first (like escape key)
		if let Some(transition) = self.handle_common_input() {
			return Some(transition);
		}
		self.player1.update();
		self.player2.update();

		if self.player1.collider().unwrap()
				.overlaps(&self.player2.collider().unwrap()) {
			self.player1.on_player_hit(&mut self.player2);
		}

		if is_key_down(PLAYER1_SHOOT) {
			if let Some(bullet) = self.player1.shoot() {
				self.bullets.push(bullet);
			}
		}

		if is_key_down(PLAYER2_SHOOT) {
			if let Some(bullet) = self.player2.shoot() {
				self.bullets.push(bullet);
			}
		}

		for bullet in &mut self.bullets {
			bullet.update();
		}

		// Remove bullets that are out of bounds, or hit a player
		self.bullets.retain(|bullet| {
			let pos = bullet.pos;
			!bullet.destroy_flag && (
				pos.x > 0.0 &&
				pos.x < screen_width() &&
				pos.y > 0.0 &&
				pos.y < screen_height()
			)
		});

		// Check for bullet collisions
		for bullet in &mut self.bullets {
			if bullet.color == PlayerColor::Blue &&
					bullet.collider().unwrap().overlaps(&self.player1.collider().unwrap()) {
				self.player1.on_bullet_hit(bullet);
				bullet.destroy_flag = true;
			}
			else if bullet.color == PlayerColor::Red &&
					bullet.collider().unwrap().overlaps(&self.player2.collider().unwrap()) {
				self.player2.on_bullet_hit(bullet);
				bullet.destroy_flag = true;
			}
		}
		
		None
	}

	fn render(&mut self) {
		// TODO: background
		clear_background(BLACK);
		
		self.player1.render();
		self.player2.render();
		for bullet in &self.bullets {
			bullet.render();
		}
	}
}
