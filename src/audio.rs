use std::io::Cursor;

use rodio::{Decoder, Sink, Source};

const MENU_THEME: &[u8] = include_bytes!("../assets/bgm/menu.ogg");
pub const GAME_THEMES: [&[u8]; 4] = [
	include_bytes!("../assets/bgm/game-a.ogg"),
	include_bytes!("../assets/bgm/game-b.ogg"),
	include_bytes!("../assets/bgm/game-c.ogg"),
	include_bytes!("../assets/bgm/game-d.ogg"),
];
const SFX: [&[u8]; 3] = [
	include_bytes!("../assets/sfx/button-click.wav"),
	include_bytes!("../assets/sfx/shoot.wav"),
	include_bytes!("../assets/sfx/hit.wav"),
];

#[derive(PartialEq, Clone, Copy)]
pub enum Theme {
	Menu,
	Game(usize),
}

#[derive(PartialEq, Clone, Copy)]
pub enum Effect {
	ButtonClick,
	Shoot,
	Hit,
}

pub struct AudioPlayer {
	bgm_queue: Sink,
	sfx_volume: f32,
	_stream: rodio::OutputStream,
	stream_handle: rodio::OutputStreamHandle,
}

impl Default for AudioPlayer {
	fn default() -> Self {
		Self::new()
	}
}

impl AudioPlayer {
	pub fn new() -> Self {
		let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
		Self {
			bgm_queue: Sink::try_new(&stream_handle).unwrap(),
			sfx_volume: 1.0,
			_stream,
			stream_handle,
		}
	}

	pub fn set_sfx_volume(&mut self, volume: f32) {
		self.sfx_volume = volume;
	}

	pub fn sfx_volume(&self) -> f32 {
		self.sfx_volume
	}

	pub fn set_bgm_volume(&self, volume: f32) {
		self.bgm_queue.set_volume(volume);
	}

	pub fn bgm_volume(&self) -> f32 {
		self.bgm_queue.volume()
	}

	pub fn stop_bgm(&self) {
		self.bgm_queue.stop();
	}

	fn get_bgm(&self, theme: Theme) -> Cursor<&'static [u8]> {
		Cursor::new(match theme {
			Theme::Menu => MENU_THEME,
			Theme::Game(index) => GAME_THEMES[index],
		})
	}

	fn get_sfx(&self, effect: Effect) -> Cursor<&'static [u8]> {
		Cursor::new(match effect {
			Effect::ButtonClick => SFX[0],
			Effect::Shoot => SFX[1],
			Effect::Hit => SFX[2],
		})
	}

	pub fn play_bgm_loop(&self, theme: Theme) {
		let source = Decoder::new(self.get_bgm(theme)).unwrap();
		let source = source.repeat_infinite();
		self.bgm_queue.append(source);
	}

	pub fn queue_bgm(&self, theme: Theme) {
		let source = Decoder::new(self.get_bgm(theme)).unwrap();
		self.bgm_queue.append(source);
	}

	pub fn done_bgm(&self) -> bool {
		self.bgm_queue.empty()
	}

	pub fn play_sfx(&self, effect: Effect) {
		let source = Decoder::new(self.get_sfx(effect)).unwrap();
		self.stream_handle
			.play_raw(source.amplify(self.sfx_volume).convert_samples())
			.unwrap();
	}
}
