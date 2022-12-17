use level::load_gd_level_string;
use sfml::graphics::{
	Color, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
};
use sfml::system::{Vector2, Vector2f};
use sfml::window::{Event, Key};

mod level;
mod player;

use player::{AxisBoundingBox, Player, OBJECT_SIZE};

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
	let mut window = sfml::graphics::RenderWindow::new(
		(896, 504),
		"Hewwo",
		Default::default(),
		&Default::default(),
	);
	window.set_framerate_limit(60);

	let texture = sfml::graphics::Texture::from_file("player.png").unwrap();

	let mut player = Player::new();
	player.x = 100.0;

	let objects = load_gd_level_string(std::fs::read_to_string("stereo.txt").unwrap().as_str());

	while window.is_open() {
		while let Some(ev) = window.poll_event() {
			match ev {
				Event::Closed => window.close(),
				_ => {}
			}
		}

		if window.has_focus() {
			if Key::is_pressed(Key::Up)
				|| sfml::window::mouse::Button::is_pressed(sfml::window::mouse::Button::Left)
			{
				player.is_holding = true;
				player.jump();
			} else {
				player.is_holding = false;
			}
			if Key::is_pressed(Key::R) {
				player.reset();
			}
		}

		player.update(1.0 / 60.0, objects.as_slice());

		let window_size: Vector2<f32> = window.size().as_other();
		// fit 11 objects vertically
		let scale = window_size.y / (OBJECT_SIZE * 11.0);
		let scaled_window_size = window_size / (2.0 * scale);
		let mut my_view = sfml::graphics::View::new(
			Vector2f::new(
				scaled_window_size.x + player.x - window_size.x / 6.0,
				window_size.y - scaled_window_size.y + 10.0,
			),
			window_size / scale,
		);
		let following_player_y =
			window_size.y - scaled_window_size.y - player.y + scaled_window_size.y;
		if following_player_y < my_view.center().y {
			let center = my_view.center();
			my_view.set_center(Vector2f::new(center.x, following_player_y));
		}
		window.set_view(&my_view);

		window.clear(Color::BLACK);

		let size = OBJECT_SIZE;
		let mut shape = RectangleShape::from_rect(Rect::new(
			player.x,
			window.size().y as f32 - player.y,
			size,
			size,
		));
		shape.set_texture(&texture, true);
		shape.set_origin((size / 2.0, size / 2.0));
		shape.set_rotation(player.rotation);

		window.draw(&shape);

		draw_box(&mut window, &player.bounding_box(), Color::GREEN);
		draw_box(&mut window, &player.inner_bounding_box(), Color::BLUE);

		for object in &objects {
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
