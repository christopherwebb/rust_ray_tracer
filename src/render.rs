use serde::{Deserialize, Serialize};

// use crate::vector::Vec3;
use crate::core::Colour;


#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct RenderResult {
    pub x_coord: f32,
    pub y_coord: f32,
    pub time: f32,
    pub colour: Colour,
}
