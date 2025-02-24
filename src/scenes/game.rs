use std::f32::consts::PI;

use macroquad::prelude::*;

use super::{Scene, SceneBehavior, Transition};

pub struct Game {
	player_pos: Vec2,
	player_vel: Vec2,
	player_speed: f32,
}

const PLAYER_RADIUS: f32 = 40.0;

impl Game {
	pub fn new() -> Self {
		Self {
			player_pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
			player_vel: Vec2::new(0.0, 0.0),
			player_speed: 0.2,
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

		let mut movement = Vec2::ZERO;

		if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
			movement.y -= 1.0;
		}
		if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
			movement.y += 1.0;
		}
		if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
			movement.x -= 1.0;
		}
		if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
			movement.x += 1.0;
		}

		if movement != Vec2::ZERO {
			movement = movement.normalize();
			self.player_vel += movement * self.player_speed;
		}
		self.player_pos += self.player_vel;
		self.player_vel /= 1.05;

		// Keep player within screen bounds, accounting for radius
		self.player_pos.x = self
			.player_pos
			.x
			.clamp(PLAYER_RADIUS, screen_width() - PLAYER_RADIUS);
		self.player_pos.y = self
			.player_pos
			.y
			.clamp(PLAYER_RADIUS, screen_height() - PLAYER_RADIUS);

		None
	}

	fn render(&mut self) {
		clear_background(BLACK);
		let rad = 7.0 * self.player_vel.y / 360.0;
		draw_rectangle_ex(self.player_pos.x - 0.5f32.sqrt() * PLAYER_RADIUS * (rad + PI/4.0).cos(), self.player_pos.y - 0.5f32.sqrt() * PLAYER_RADIUS * (rad + PI/4.0).sin(), PLAYER_RADIUS, PLAYER_RADIUS, DrawRectangleParams {
			color: RED,
			rotation: rad,
			..Default::default()
		});
		draw_rectangle_ex(self.player_pos.x + 5.0 * (rad).sin(), self.player_pos.y - 5.0 * (rad).cos(), 30.0, 10.0, DrawRectangleParams {
			color: RED,
			rotation: rad,
			..Default::default()
		});
		draw_rectangle_ex(screen_width() - self.player_pos.x + 0.5f32.sqrt() * PLAYER_RADIUS * (rad + PI/4.0).cos(), self.player_pos.y - 0.5f32.sqrt() * PLAYER_RADIUS * (rad + PI/4.0).sin(), -PLAYER_RADIUS, PLAYER_RADIUS, DrawRectangleParams {
			color: BLUE,
			rotation: -rad,
			..Default::default()
		});
		draw_rectangle_ex(screen_width() - self.player_pos.x + 5.0 * (rad).sin(), self.player_pos.y - 5.0 * (rad).cos(), -30.0, 10.0, DrawRectangleParams {
			color: BLUE,
			rotation: -rad,
			..Default::default()
		});
	}
}
