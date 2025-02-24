use macroquad::prelude::*;

mod bullet;
mod player;

pub use self::bullet::Bullet;
pub use self::player::Player;
pub use self::player::PlayerColor;

pub trait Entity {
	fn update(&mut self);
	fn render(&self);
	fn collider(&self) -> Option<Rect>;
}
