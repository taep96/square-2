use macroquad::prelude::*;

use super::{player::PlayerColor, Entity};

const BULLET_RADIUS: f32 = 5.0;

pub enum BulletType {
	Regular,
	Charged
}

pub struct Bullet {
	pub color: PlayerColor,
	pub pos: Vec2,
	pub destroy_flag: bool,
	bullet_type: BulletType,
	vel: Vec2,
}

impl Bullet {
	pub fn new(color: PlayerColor, pos: Vec2, vel: Vec2, bullet_type: BulletType) -> Self {
		Self {
			color,
			pos,
			destroy_flag: false,
			vel,
			bullet_type,
		}
	}

	pub fn damage(&self) -> u32 {
		match self.bullet_type {
			BulletType::Regular => 1,
			BulletType::Charged => 3,
		}
	}
}

impl Entity for Bullet {
	fn collider(&self) -> Option<Rect> {
		Some(Rect { 
			x: self.pos.x - BULLET_RADIUS,
			y: self.pos.y - BULLET_RADIUS,
			w: 2.0 * BULLET_RADIUS,
			h: 2.0 * BULLET_RADIUS,
		})
	}

	fn render(&self) {
		draw_circle(self.pos.x, self.pos.y, BULLET_RADIUS, match self.color {
			PlayerColor::Red => RED,
			PlayerColor::Blue => BLUE,
		});

		// debug
		// let Rect { x, y, w, h } = self.collider().unwrap();
		// draw_rectangle_lines(x, y, w, h, 2.0, WHITE);
	}

	fn update(&mut self) {
		let delta = get_frame_time();
		self.pos += self.vel * delta;
	}
}