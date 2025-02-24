use macroquad::prelude::*;

mod player;
mod bullet;

pub use self::player::Player;
pub use self::player::PlayerColor;
pub use self::bullet::Bullet;

pub trait Entity {
	fn update(&mut self);
	fn render(&self);
	fn collider(&self) -> Option<Rect>;
}