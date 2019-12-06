
use std::ops::*;

use crate::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
  vec1: Vector3,
  vec2: Vector3
}

impl Ray {
  pub fn new(vec1: Vector3, vec2: Vector3) -> Ray {
    return Ray{vec1, vec2};
  }

  pub fn origin(&self) -> Vector3 { self.vec1 }
  pub fn direction(&self) -> Vector3 { self.vec2 }
  pub fn point_at_direction(&self, t:f32) -> Vector3 { self.vec1 + t * self.vec2}
}
