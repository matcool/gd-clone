#[derive(Debug, Clone)]
pub struct AxisBoundingBox {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl AxisBoundingBox {
	pub fn intersects(&self, other: &Self) -> bool {
		let hx = self.x + self.width / 2.0;
		let hy = self.y - self.height / 2.0;
		let hxo = other.x + other.width / 2.0;
		let hyo = other.y - other.height / 2.0;
		(hx - hxo).abs() <= (self.width + other.width) / 2.0
			&& (hy - hyo).abs() <= (self.height + other.height) / 2.0
	}

	pub fn offset_by(&self, x: f32, y: f32) -> Self {
		Self {
			x: self.x + x,
			y: self.y + y,
			width: self.width,
			height: self.height,
		}
	}
}

pub struct Object {
	pub x: f32,
	pub y: f32,
	pub bounding_box: AxisBoundingBox,
	pub death: bool,
}

pub const OBJECT_SIZE: f32 = 30.0;
pub const HALF_OBJECT_SIZE: f32 = OBJECT_SIZE / 2.0;

impl Object {
	pub fn new() -> Self {
		Self {
			x: 0.0,
			y: HALF_OBJECT_SIZE,
			bounding_box: AxisBoundingBox {
				x: -HALF_OBJECT_SIZE,
				y: HALF_OBJECT_SIZE,
				width: OBJECT_SIZE,
				height: OBJECT_SIZE,
			},
			death: false,
		}
	}
	pub fn offset_bounding_box(&self) -> AxisBoundingBox {
		self.bounding_box.offset_by(self.x, self.y)
	}
}

pub struct Player {
	pub x: f32,
	pub y: f32,
	pub rotation: f32,
	pub y_vel: f32,
	rotation_vel: f32,
	pub dead: bool,
	on_ground: bool,
}

impl Player {
	pub fn new() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			rotation: 0.0,
			y_vel: 0.0,
			rotation_vel: 360.0,
			dead: false,
			on_ground: false,
		}
	}

	pub fn bounding_box(&self) -> AxisBoundingBox {
		AxisBoundingBox {
			x: self.x - HALF_OBJECT_SIZE,
			y: self.y + HALF_OBJECT_SIZE,
			width: OBJECT_SIZE,
			height: OBJECT_SIZE,
		}
	}

	pub fn inner_bounding_box(&self) -> AxisBoundingBox {
		let size = OBJECT_SIZE / 4.0;
		AxisBoundingBox {
			x: self.x - size / 2.0,
			y: self.y + size / 2.0,
			width: size,
			height: size,
		}
	}

	pub fn update(&mut self, dt: f32, objects: &[Object]) {
		if self.dead {
			return;
		}
		const SUBSTEPS: i32 = 4;
		let dt = dt / SUBSTEPS as f32;
		for _ in 0..SUBSTEPS {
			let mut ground = 0.0;
			for object in objects {
				let object_bb = object.offset_bounding_box();
				if self.bounding_box().intersects(&object_bb) {
					if object.death {
						self.dead = true;
						return;
					}
					// only "step" up on half block diff
					if self.y - HALF_OBJECT_SIZE
						>= object.y
							+ (object.bounding_box.height / 2.0 - OBJECT_SIZE / 3.0).max(0.0)
					{
						if object.offset_bounding_box().y > ground && self.y_vel < 50.0 {
							ground = object.offset_bounding_box().y;
						}
					} else if self.inner_bounding_box().intersects(&object_bb) {
						self.dead = true;
						return;
					}
				}
			}
			// FIXME: this all sux, 2220 and 500 make no sense
			self.y += self.y_vel * dt;
			if self.y - HALF_OBJECT_SIZE <= ground {
				self.y = ground + HALF_OBJECT_SIZE;
				self.y_vel = 0.0;
				self.rotation = (self.rotation / 90.0).round() * 90.0;
				self.on_ground = true;
			} else {
				self.on_ground = false;
				self.y_vel -= 2200.0 * dt;
				self.rotation += self.rotation_vel * dt;
			}
			self.x += 311.5776 * dt;
		}
	}

	pub fn jump(&mut self) {
		if self.on_ground {
			self.y_vel = 500.0;
		}
	}
}
