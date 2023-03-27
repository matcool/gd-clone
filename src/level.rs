use std::{collections::HashMap, io::Read};

use base64::Engine;

use crate::player::{AxisBoundingBox, Object, Player, HALF_OBJECT_SIZE};

fn parse_hitboxes() -> HashMap<i32, AxisBoundingBox> {
	let mut hitboxes = HashMap::new();
	let parsed: serde_json::Value =
		serde_json::from_str(std::fs::read_to_string("res/hitboxes.json").unwrap().as_str()).unwrap();
	for (key, value) in parsed.as_object().unwrap() {
		let id: i32 = key.parse().unwrap();
		let value = value.as_object().unwrap();
		hitboxes.insert(
			id,
			AxisBoundingBox {
				x: value["x"].as_f64().unwrap() as f32,
				y: -value["y"].as_f64().unwrap() as f32,
				width: value["w"].as_f64().unwrap() as f32,
				height: value["h"].as_f64().unwrap() as f32,
			},
		);
	}
	hitboxes
}

pub fn load_gd_level_string(string: &str) -> (Vec<Object>, Vec<(f32, f32)>) {
	let mut start_positions = Vec::new();
	start_positions.push((-60.0, HALF_OBJECT_SIZE));
	let mut objects = Vec::new();
	let hitboxes = parse_hitboxes();
	for object in string.split(';').skip(1) {
		let keys = object.split(',').step_by(2);
		let values = object.split(',').skip(1).step_by(2);
		let mut object = Object::new();
		let mut has_hitbox = false;
		for (key, value) in keys.zip(values) {
			match key {
				"1" => {
					let id = value.parse().unwrap();
					object.id = id;
					if id == 8 {
						object.death = true;
					}
					if let Some(hitbox) = hitboxes.get(&id) {
						has_hitbox = true;
						object.bounding_box = hitbox.clone();
					}
				}
				"2" => object.x = value.parse().unwrap(),
				"3" => object.y = value.parse().unwrap(),
				_ => {}
			}
		}
		if object.id == 31 {
			start_positions.push((object.x, object.y));
		}
		if (22..34).contains(&object.id) {
			continue;
		}
		if has_hitbox {
			objects.push(object);
		}
	}
	(objects, start_positions)
}

fn load_gmd(string: &str) -> (Vec<Object>, Vec<(f32, f32)>) {
	let pos = string.find("<k>k4</k><s>").unwrap();
	let string = &string[pos + 12..];
	let pos = string.find("</s>").unwrap();
	let string = &string[..pos].trim().trim_end_matches('=');

	let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
		.decode(string)
		.unwrap();
	let mut gz = flate2::read::GzDecoder::new(decoded.as_slice());
	let mut string = String::new();
	gz.read_to_string(&mut string).unwrap();

	load_gd_level_string(string.as_str())
}

pub struct Level {
	pub player: Player,
	pub objects: Vec<Object>,
	start_positions: Vec<(f32, f32)>,
	start_pos_index: usize,
}

impl Level {
	pub fn from_gmd(file_name: &str) -> Self {
		let (objects, start_positions) =
			load_gmd(std::fs::read_to_string(file_name).unwrap().as_str());
		Self {
			player: Player::new(),
			objects,
			start_positions,
			start_pos_index: 0,
		}
	}

	pub fn reset(&mut self) {
		self.player.reset();
		let start_pos = self.start_positions[self.start_pos_index];
		self.player.x = start_pos.0;
		self.player.y = start_pos.1;
	}

	pub fn update(&mut self, dt: f32) {
		self.player.update(dt, &self.objects);
	}

	pub fn next_start_pos(&mut self) {
		self.start_pos_index = (self.start_pos_index + 1) % self.start_positions.len();
	}
}
