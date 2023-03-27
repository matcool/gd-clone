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
		(hx - hxo).abs() < (self.width + other.width) / 2.0
			&& (hy - hyo).abs() < (self.height + other.height) / 2.0
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
	pub id: i32,
	pub solid: bool,
}

pub const OBJECT_SIZE: f32 = 30.0;
pub const HALF_OBJECT_SIZE: f32 = OBJECT_SIZE / 2.0;

impl Object {
	pub fn new() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			bounding_box: AxisBoundingBox {
				x: -HALF_OBJECT_SIZE,
				y: HALF_OBJECT_SIZE,
				width: OBJECT_SIZE,
				height: OBJECT_SIZE,
			},
			death: false,
			id: -1,
			solid: true,
		}
	}
	pub fn offset_bounding_box(&self) -> AxisBoundingBox {
		self.bounding_box.offset_by(self.x, self.y)
	}
}

#[derive(PartialEq)]
pub enum PlayerMode {
	Cube,
	Ship,
}

pub struct Player {
	pub x: f32,
	pub y: f32,
	pub rotation: f32,
	y_vel: f32,
	rotation_vel: f32,
	dead: bool,
	on_ground: bool,
	pub mode: PlayerMode,
	pub is_holding: bool,
	is_rising: bool,
	gravity: f32,
	// this should be on Level, but player doesnt have access to it yet
	pub portal_y: f32,
	// if just started clicking and can hit orb
	is_buffering: bool,
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
			mode: PlayerMode::Cube,
			is_holding: false,
			is_rising: false,
			gravity: 0.958199,
			portal_y: 0.0,
			is_buffering: false,
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
		let size = OBJECT_SIZE * 0.3;
		AxisBoundingBox {
			x: self.x - size / 2.0,
			y: self.y + size / 2.0,
			width: size,
			height: size,
		}
	}

	pub fn ground_height(&self) -> f32 {
		if self.mode == PlayerMode::Ship {
			((self.portal_y / OBJECT_SIZE).floor() * OBJECT_SIZE).max(5.0 * OBJECT_SIZE)
				- 5.0 * OBJECT_SIZE
		} else {
			0.0
		}
	}

	pub fn ceiling_height(&self) -> f32 {
		self.ground_height() + 10.0 * OBJECT_SIZE
	}

	pub fn update(&mut self, dt: f32, objects: &[Object]) {
		if self.dead {
			return;
		}
		const SUBSTEPS: i32 = 4;
		let dt = dt / SUBSTEPS as f32;
		for _ in 0..SUBSTEPS {
			let mut ground = self.ground_height();
			let mut ceiling = self.ceiling_height();
			for object in objects {
				let object_bb = object.offset_bounding_box();
				if self.bounding_box().intersects(&object_bb) {
					if object.death {
						self.dead = true;
						return;
					}
					if object.id == 13 {
						self.mode = PlayerMode::Ship;
						self.rotation = 0.0;
						self.portal_y =
							((object.y / OBJECT_SIZE).floor() * OBJECT_SIZE).max(OBJECT_SIZE * 5.0);
					}
					if object.id == 12 {
						self.mode = PlayerMode::Cube;
					}
					if object.id == 35 {
						// yellow pad, made up value
						self.y_vel = 16.0;
					}
					if object.id == 36 && self.is_buffering {
						// made up physics
						self.y_vel = 11.5;
						self.is_buffering = false;
					}
					if !object.solid {
						continue;
					}
					let player_bottom = self.y - HALF_OBJECT_SIZE;
					// only step up on 1/3 of a block
					let object_top =
						object.y + (object_bb.height / 2.0 - OBJECT_SIZE / 3.0).max(0.0);

					let player_top = self.y + HALF_OBJECT_SIZE;
					let object_bottom = object_bb.y - object_bb.height;

					if player_top < object_top && object_bottom < ceiling {
						ceiling = object_bottom;
					}

					if player_bottom >= object_top {
						if object_bb.y > ground && self.y_vel < 1.0 {
							ground = object_bb.y;
						}
					} else if self.inner_bounding_box().intersects(&object_bb) {
						self.dead = true;
						return;
					}
				}
			}

			let rob_dt = dt * 60.0;
			let slow_dt = rob_dt * 0.9;

			// for 1x
			let player_speed = 0.9;
			let x_velocity = 5.7700018;

			self.update_jump(slow_dt);

			let dx = x_velocity * player_speed * rob_dt;
			let dy = self.y_vel * slow_dt;
			self.x += dx;
			self.y += dy;

			if self.y - HALF_OBJECT_SIZE <= ground {
				self.y = ground + HALF_OBJECT_SIZE;
				self.y_vel = 0.0;
				if self.mode == PlayerMode::Cube {
					self.rotation = (self.rotation / 90.0).round() * 90.0;
				} else {
					self.rotation = 0.0;
				}
				self.on_ground = true;
			} else if self.mode == PlayerMode::Ship
				&& self.y + HALF_OBJECT_SIZE >= ceiling
				&& self.y_vel > 0.0
			{
				self.y = ceiling - HALF_OBJECT_SIZE;
				self.y_vel = 0.0;
				self.rotation = 0.0;
			} else {
				self.on_ground = false;
				if self.mode == PlayerMode::Cube {
					self.rotation += self.rotation_vel * dt;
				} else {
					// dx dy tan-1 to get angle
					self.rotation = -(dy / dx).atan().to_degrees();
				}
			}
		}
	}

	pub fn reset(&mut self) {
		self.dead = false;
		self.y_vel = 0.0;
		self.mode = PlayerMode::Cube;
	}

	fn update_jump(&mut self, slow_dt: f32) {
		let local_gravity = self.gravity;
		// TODO: gravity is fixed for everything not cube

		let flip_gravity = 1.0; // -1.0 when upside down
		let player_size = 1.0;

		match self.mode {
			PlayerMode::Cube => {
				let jump_power = 11.180032; // m_jumpAccel

				let gravity_multiplier = 1.0;

				let should_jump = self.is_holding;

				if self.on_ground {
					self.is_buffering = false;
				}

				if should_jump && self.on_ground {
					self.on_ground = false;
					self.is_rising = true;

					self.y_vel = flip_gravity * jump_power * player_size;
				} else {
					if self.is_rising {
						self.y_vel -= local_gravity * slow_dt * flip_gravity * gravity_multiplier;

						if self.y_vel <= self.gravity * 2.0 {
							self.is_rising = false;
						}
					} else {
						if self.y_vel < self.gravity * 2.0 {
							self.on_ground = false;
						}
						self.y_vel -= local_gravity * slow_dt * flip_gravity * gravity_multiplier;
						if self.y_vel <= -15.0 {
							self.y_vel = -15.0;
						}
					}
				}
			}
			PlayerMode::Ship => {
				let upper_velocity = 8.0 / player_size;
				let lower_velocity = -6.4 / player_size;

				let mut ship_accel = 0.8;
				if self.is_holding {
					ship_accel = -1.0;
				}

				if !self.is_holding && !self.is_falling() {
					ship_accel = 1.2;
				}

				let mut extra_boost = 0.4;
				if self.is_holding && self.is_falling() {
					extra_boost = 0.5;
				}

				self.y_vel -=
					local_gravity * slow_dt * flip_gravity * ship_accel * extra_boost / player_size;

				self.y_vel = self.y_vel.clamp(lower_velocity, upper_velocity);
			}
		}
	}

	fn is_falling(&self) -> bool {
		self.y_vel < self.gravity * 2.0
	}

	pub fn press_jump(&mut self) {
		self.is_holding = true;
		self.is_buffering = true;
	}

	pub fn release_jump(&mut self) {
		self.is_holding = false;
		self.is_buffering = false;
	}
}
