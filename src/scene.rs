use serde::{Deserialize, Serialize};

use crate::camera::Camera;
use crate::world::HitList;


#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
	pub hitlist : HitList,
	pub camera : Camera,
}
