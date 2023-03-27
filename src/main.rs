use level::Level;
use sfml::graphics::{
	Color, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
};
use sfml::system::{Vector2, Vector2f};
use sfml::window::{Event, Key};

mod level;
mod player;

use player::{AxisBoundingBox, OBJECT_SIZE};

fn draw_box(window: &mut RenderWindow, bounding_box: &AxisBoundingBox, color: Color) {
	let window_height = window.size().y as f32;
	let mut shape = RectangleShape::from_rect(Rect::new(
		bounding_box.x,
		window_height - bounding_box.y,
		bounding_box.width,
		bounding_box.height,
	));
	shape.set_fill_color(Color::TRANSPARENT);
	shape.set_outline_color(color);
	shape.set_outline_thickness(-2.0);

	window.draw(&shape);
}

fn main() {
	let mut level = Level::from_gmd(std::env::args().nth(1).unwrap().as_str());

	let mut window = sfml::graphics::RenderWindow::new(
		(896, 504),
		"Hewwo",
		Default::default(),
		&Default::default(),
	);
	window.set_framerate_limit(60);

	let texture = sfml::graphics::Texture::from_file("player.png").unwrap();
	let ship_texture = sfml::graphics::Texture::from_file("ship.png").unwrap();

	level.reset();

	window.set_key_repeat_enabled(false);

	while window.is_open() {
		while let Some(ev) = window.poll_event() {
			match ev {
				Event::Closed => window.close(),
				Event::KeyPressed { code: key, .. } => match key {
					Key::R => level.reset(),
					Key::Right => {
						level.next_start_pos();
						level.reset();
					}
					_ => {}
				},
				_ => {}
			}
		}

		if window.has_focus() {
			if Key::Up.is_pressed()
				|| sfml::window::mouse::Button::is_pressed(sfml::window::mouse::Button::Left)
			{
				level.player.is_holding = true;
			} else {
				level.player.is_holding = false;
			}
		}

		level.update(1.0 / 60.0);

		let window_size: Vector2<f32> = window.size().as_other();
		// fit 11 objects vertically
		let scale = window_size.y / (OBJECT_SIZE * 11.0);
		let scaled_window_size = window_size / (2.0 * scale);
		let mut my_view = sfml::graphics::View::new(
			Vector2f::new(
				scaled_window_size.x + level.player.x - scaled_window_size.x / 2.0,
				window_size.y - scaled_window_size.y + 10.0,
			),
			window_size / scale,
		);
		let following_player_y =
			window_size.y - scaled_window_size.y - level.player.y + scaled_window_size.y;
		if following_player_y < my_view.center().y {
			let center = my_view.center();
			my_view.set_center(Vector2f::new(center.x, following_player_y));
		}
		window.set_view(&my_view);

		window.clear(Color::BLACK);

		let size = OBJECT_SIZE;
		let mut shape = RectangleShape::from_rect(Rect::new(
			level.player.x,
			window.size().y as f32 - level.player.y,
			size,
			size,
		));
		match level.player.mode {
			player::PlayerMode::Cube => shape.set_texture(&texture, true),
			player::PlayerMode::Ship => shape.set_texture(&ship_texture, false),
		}
		shape.set_origin((size / 2.0, size / 2.0));
		shape.set_rotation(level.player.rotation);

		window.draw(&shape);

		draw_box(&mut window, &level.player.bounding_box(), Color::GREEN);
		draw_box(&mut window, &level.player.inner_bounding_box(), Color::BLUE);

		for object in &level.objects {
			if object.x > my_view.center().x + my_view.size().x / 2.0 {
				continue;
			}
			let color = if object.death {
				Color::RED
			} else {
				Color::BLUE
			};
			draw_box(&mut window, &object.offset_bounding_box(), color);
		}

		window.display();
	}
}
