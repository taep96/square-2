use crate::scenes::Transition;
use macroquad::prelude::*;

/// Style configuration for a button
#[derive(Clone, Copy)]
pub struct ButtonStyle {
	/// Normal background color
	pub background: Color,
	/// Background color when hovered
	pub hover_background: Color,
	/// Text color
	pub text_color: Color,
}

impl ButtonStyle {
	/// Creates a dark style
	pub fn dark() -> Self {
		Self {
			background: Color::new(0.2, 0.2, 0.2, 1.0),
			hover_background: Color::new(0.3, 0.3, 0.3, 1.0),
			text_color: WHITE,
		}
	}
}

impl Default for ButtonStyle {
	fn default() -> Self {
		Self::dark()
	}
}

/// A clickable button with customizable appearance and behavior
pub struct Button {
	/// The text displayed on the button
	label: String,
	/// The button's position and size in viewport percentage (0-100)
	bounds: Rect,
	/// Whether the mouse is currently hovering over the button
	hovering: bool,
	/// The callback function when the button is clicked
	on_click: Box<dyn Fn() -> Option<Transition>>,
	/// Custom styling for the button
	style: ButtonStyle,
}

impl Button {
	/// Creates a new button with default dark styling
	pub fn new(
		label: impl Into<String>,
		bounds: Rect,
		on_click: impl Fn() -> Option<Transition> + 'static,
	) -> Self {
		Button {
			label: label.into(),
			bounds,
			hovering: false,
			on_click: Box::new(on_click),
			style: ButtonStyle::default(),
		}
	}

	/// Creates a menu-style button with default dark styling
	pub fn menu_button(
		label: impl Into<String>,
		y_offset_percent: f32,
		on_click: impl Fn() -> Option<Transition> + 'static,
	) -> Self {
		let x = 35.0; // Percentage from left
		Self::new(
			label,
			Rect {
				x,
				y: y_offset_percent,
				w: 30.0, // Percentage of screen width
				h: 8.0,  // Percentage of screen height
			},
			on_click,
		)
	}

	/// Updates the button state and handles interactions
	pub fn update(&mut self) -> Option<Transition> {
		let bounds = self.get_absolute_bounds();
		let (cx, cy) = mouse_position();

		// Update hover state
		self.hovering = cx >= bounds.x
			&& cx <= bounds.x + bounds.w
			&& cy >= bounds.y
			&& cy <= bounds.y + bounds.h;

		// Handle click
		if self.hovering && is_mouse_button_pressed(MouseButton::Left) {
			return (self.on_click)();
		}

		None
	}

	/// Renders the button
	pub fn render(&mut self) {
		let bounds = self.get_absolute_bounds();

		// Draw background
		draw_rectangle(
			bounds.x,
			bounds.y,
			bounds.w,
			bounds.h,
			if self.hovering {
				self.style.hover_background
			} else {
				self.style.background
			},
		);

		// Calculate text dimensions for centering
		let font_size = bounds.h * 0.6;
		let text_dims = measure_text(&self.label, None, font_size as u16, 1.0);

		// Draw centered text
		draw_text(
			&self.label,
			bounds.x + (bounds.w - text_dims.width) * 0.5,
			bounds.y + (bounds.h + text_dims.height) * 0.5,
			font_size,
			self.style.text_color,
		);
	}

	/// Converts the button's viewport percentage bounds to screen coordinates
	fn get_absolute_bounds(&self) -> Rect {
		let vw = screen_width() / 100.0;
		let vh = screen_height() / 100.0;

		Rect {
			x: self.bounds.x * vw,
			y: self.bounds.y * vh,
			w: self.bounds.w * vw,
			h: self.bounds.h * vh,
		}
	}
}
