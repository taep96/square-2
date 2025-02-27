use std::rc::Rc;

use macroquad::prelude::*;

use crate::audio::{AudioPlayer, Effect};

use super::{bullet::BulletType, Bullet, Entity};

const PLAYER_SIDE: f32 = 80.0;
const PLAYER_LIVES: u32 = 100;
const ROT_RATE: f32 = 1.0 / 60.0; // degrees per px/s
const DRAG: f32 = 4.0; // relative slowdown per second
const FORCE: f32 = 1000.0; // (m)*px/s^2
const BOUNCE_FORCE: f32 = 200.0;
const INITIAL_BULLET_VEL: f32 = 500.0;
const COOLDOWN: f32 = 0.1; // seconds

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerColor {
	Red,
	Blue,
}

pub struct Player {
	pub color: PlayerColor,
	pub lives: u32,
	pos: Vec2,
	vel: Vec2,
	shoot_cooldown: f32,
	audio_player: Rc<AudioPlayer>,
}

impl Player {
	pub fn new(color: PlayerColor, audio_player: Rc<AudioPlayer>) -> Self {
		let x_start = match color {
			PlayerColor::Red => 0.25,
			PlayerColor::Blue => 0.75,
		} * screen_width();
		Self {
			color,
			pos: Vec2::new(x_start, screen_height() / 2.0),
			vel: Vec2::new(0.0, 0.0),
			lives: PLAYER_LIVES,
			shoot_cooldown: 0.0,
			audio_player,
		}
	}

	pub fn on_bullet_hit(&mut self, _: &Bullet) {
		self.audio_player.play_sfx(Effect::Hit);
		if self.lives != 0 {
			self.lives -= 1;
		}
	}

	pub fn on_player_hit(&mut self, player: &mut Player) {
		let collision_normal = (self.pos - player.pos).normalize();
		self.vel += collision_normal * BOUNCE_FORCE;
		player.vel -= collision_normal * BOUNCE_FORCE;

		self.audio_player.play_sfx(Effect::Collision);
	}

	fn acceleration_from_input(&self) -> Vec2 {
		let mut acc = Vec2::new(0.0, 0.0);
		if self.color == PlayerColor::Red {
			if is_key_down(KeyCode::W) {
				acc.y -= 1.0;
			}
			if is_key_down(KeyCode::S) {
				acc.y += 1.0;
			}
			if is_key_down(KeyCode::A) {
				acc.x -= 1.0;
			}
			if is_key_down(KeyCode::D) {
				acc.x += 1.0;
			}
		} else {
			if is_key_down(KeyCode::Up) {
				acc.y -= 1.0;
			}
			if is_key_down(KeyCode::Down) {
				acc.y += 1.0;
			}
			if is_key_down(KeyCode::Left) {
				acc.x -= 1.0;
			}
			if is_key_down(KeyCode::Right) {
				acc.x += 1.0;
			}
		}
		FORCE * acc.normalize_or_zero()
	}

	/// Get the rotation of the player **in degrees**
	fn get_rotation(&self) -> f32 {
		(match self.color {
			PlayerColor::Red => 1.0,
			PlayerColor::Blue => -1.0,
		} * (ROT_RATE * self.vel.y))
	}

	/// generate a bullet with the player's color, position and initial velocity
	pub fn shoot(&mut self) -> Option<Bullet> {
		if self.shoot_cooldown > 0.0 {
			return None;
		}
		let rot = self.get_rotation().to_radians();
		let bullet_vel = (match self.color {
			PlayerColor::Red => Vec2::new(rot.cos(), rot.sin()),
			PlayerColor::Blue => Vec2::new(-rot.cos(), -rot.sin()),
		}) * INITIAL_BULLET_VEL
			+ Vec2 {
				x: self.vel.x,
				y: 0.0,
			};

		self.shoot_cooldown = COOLDOWN;
		self.audio_player.play_sfx(Effect::Shoot);

		Some(Bullet::new(
			self.color,
			self.pos,
			bullet_vel,
			BulletType::Regular,
		))
	}

	fn collide_wall(&mut self) {
		let half_side = PLAYER_SIDE / 2.0;
		if self.pos.x - half_side < 0.0 {
			self.pos.x = half_side;
			self.vel = Vec2::new(-self.vel.x, self.vel.y);
			self.audio_player.play_sfx(Effect::Collision);
		}
		if self.pos.x + half_side > screen_width() {
			self.pos.x = screen_width() - half_side;
			self.vel = Vec2::new(-self.vel.x, self.vel.y);
			self.audio_player.play_sfx(Effect::Collision);
		}
		if self.pos.y - half_side < 0.0 {
			self.pos.y = half_side;
			self.vel = Vec2::new(self.vel.x, -self.vel.y);
			self.audio_player.play_sfx(Effect::Collision);
		}
		if self.pos.y + half_side > screen_height() {
			self.pos.y = screen_height() - half_side;
			self.vel = Vec2::new(self.vel.x, -self.vel.y);
			self.audio_player.play_sfx(Effect::Collision);
		}
	}
}

impl Entity for Player {
	fn collider(&self) -> Option<Rect> {
		Some(Rect {
			x: self.pos.x - PLAYER_SIDE / 2.0,
			y: self.pos.y - PLAYER_SIDE / 2.0,
			w: PLAYER_SIDE,
			h: PLAYER_SIDE,
		})
	}
	fn render(&self) {
		let color = match self.color {
			PlayerColor::Red => RED,
			PlayerColor::Blue => BLUE,
		};

		draw_poly(
			self.pos.x,
			self.pos.y,
			4,
			PLAYER_SIDE / 1.5,          // slightly smaller than the collider
			self.get_rotation() + 45.0, // corner up by default
			color,
		);

		let text_size = measure_text(&format!("{0}", self.lives), None, 10, 1.0);
		let scale = (PLAYER_SIDE / text_size.width)
			.min(PLAYER_SIDE / (text_size.height + text_size.offset_y));
		draw_text(
			&format!("{0}", self.lives),
			self.pos.x - text_size.width / 2.0 * scale,
			self.pos.y + text_size.offset_y / 2.0 * scale,
			10.0 * scale,
			WHITE,
		);
		// debug
		// draw_circle(self.pos.x, self.pos.y, 4.0, WHITE);
		// let Rect { x: clx, y: cly, w: clw, h: clh } = self.collider().unwrap();
		// draw_rectangle_lines(clx, cly, clw, clh, 2.0, WHITE);
	}

	fn update(&mut self) {
		let delta = get_frame_time();
		let acc = self.acceleration_from_input();
		self.vel += acc * delta;
		self.vel /= DRAG.powf(delta);

		self.pos += self.vel * delta;

		self.collide_wall();

		self.shoot_cooldown -= delta;
		self.shoot_cooldown = self.shoot_cooldown.max(0.0);
	}
}
