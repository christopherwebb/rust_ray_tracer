use crate::core::{Point3f, Normal3f, Colour};
use crate::textures::base::Texture;


pub struct SolidColour {
	pub colour: Colour,
}

impl Texture for SolidColour {
	fn value(&self, u: f32, v: f32, p: Point3f) -> Colour {
		self.colour
	}
}