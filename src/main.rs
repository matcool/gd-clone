use level::Level;
use sfml::graphics::{
	Color, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
};
use sfml::system::{Vector2, Vector2f};
use sfml::window::{mouse, Event, Key};

mod level;
mod player;

use player::{AxisBoundingBox, PlayerMode, OBJECT_SIZE};

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
	shape.set_outline_thickness(-1.5);

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

	let texture = sfml::graphics::Texture::from_file("res/player.png").unwrap();
	let ship_texture = sfml::graphics::Texture::from_file("res/ship.png").unwrap();

	level.reset();

	window.set_key_repeat_enabled(false);

	let mut was_pressing = false;

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
			if Key::Up.is_pressed() || mouse::Button::is_pressed(mouse::Button::Left) {
				if !was_pressing {
					level.player.press_jump();
				}
				was_pressing = true;
			} else {
				if was_pressing {
					level.player.release_jump();
				}
				was_pressing = false;
			}
		}

		level.update(1.0 / 60.0);

		let window_size: Vector2<f32> = window.size().as_other();
		let scale = window_size.y / (OBJECT_SIZE * 10.588);
		let scaled_window_size = window_size / scale;

		let camera_x = level.player.x + scaled_window_size.x / 2.0 - 7.0 * OBJECT_SIZE;
		let mut camera_y =
			(scaled_window_size.y / 2.0 - 3.0 * OBJECT_SIZE).max(level.player.y - OBJECT_SIZE);

		if level.player.mode == PlayerMode::Ship {
			camera_y = level.player.portal_y.max(5.0 * OBJECT_SIZE);
		}

		let my_view = sfml::graphics::View::new(
			Vector2f::new(camera_x, window_size.y - camera_y),
			window_size / scale,
		);
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
			if object.x > my_view.center().x + my_view.size().x / 2.0
				|| object.x < my_view.center().x - my_view.size().x / 2.0
			{
				continue;
			}
			let color = if object.death {
				Color::RED
			} else if !object.solid {
				Color::YELLOW
			} else {
				Color::BLUE
			};
			draw_box(&mut window, &object.offset_bounding_box(), color);
		}

		window.display();
	}
}
