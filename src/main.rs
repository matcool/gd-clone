use sfml::graphics::{
	Color, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
};
use sfml::system::Vector2;
use sfml::window::{Event, Key};

mod player;

use player::{AxisBoundingBox, Object, Player, OBJECT_SIZE, HALF_OBJECT_SIZE};

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
	let mut objects: Vec<Object> = Vec::new();

	{
		let mut object = Object::new();
		object.x = 6.0 * OBJECT_SIZE;
		object.y = HALF_OBJECT_SIZE;
		objects.push(object);

		// let mut object = Object::new();
		// object.x = 6.0 * OBJECT_SIZE;
		// object.y = HALF_OBJECT_SIZE + OBJECT_SIZE;
		// objects.push(object);

		let mut object = Object::new();
		object.x = 7.0 * OBJECT_SIZE;
		object.y = HALF_OBJECT_SIZE;
		objects.push(object);

		for i in 0..3 {
			let mut object = Object::new();
			object.x = (20.0 + i as f32) * OBJECT_SIZE;
			object.y = HALF_OBJECT_SIZE;
			object.death = true;
			object.bounding_box = AxisBoundingBox {
				x: -3.0,
				y: 6.0,
				width: 6.0,
				height: 12.0,
			};
			objects.push(object);
		}
	}

	while window.is_open() {
		while let Some(ev) = window.poll_event() {
			match ev {
				Event::Closed => window.close(),
				_ => {}
			}
		}

		if Key::is_pressed(Key::Up) {
			player.jump();
		}
		if Key::is_pressed(Key::R) {
			player.x = 0.0;
			player.y = 0.0;
			player.dead = false;
			player.y_vel = 0.0;
		}

		player.update(1.0 / 60.0, objects.as_slice());

		let window_size: Vector2<f32> = window.size().as_other();
		let scale = 2.0;
		let scaled_window_size = window_size / (2.0 * scale);
		let my_view = sfml::graphics::View::new(
			(
				scaled_window_size.x + player.x - window_size.x / 6.0,
				window_size.y - scaled_window_size.y + 10.0,
			)
				.into(),
			window_size / scale,
		);
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
		// shape.set_fill_color(Color::GREEN);

		window.draw(&shape);

		draw_box(&mut window, &player.bounding_box(), Color::RED);

		for object in &objects {
			draw_box(&mut window, &object.offset_bounding_box(), if object.death { Color::RED } else { Color::BLUE });
		}

		window.display();
	}
}
