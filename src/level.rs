use std::collections::HashMap;

use crate::player::{AxisBoundingBox, Object};

fn parse_hitboxes() -> HashMap<i32, AxisBoundingBox> {
	let mut hitboxes = HashMap::new();
	let parsed: serde_json::Value =
		serde_json::from_str(std::fs::read_to_string("hitboxes.json").unwrap().as_str()).unwrap();
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

pub fn load_gd_level_string(string: &str) -> Vec<Object> {
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
		if (22..34).contains(&object.id) {
			continue;
		}
		if has_hitbox {
			objects.push(object);
		}
	}
	objects
}
