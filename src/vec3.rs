
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
  e: [f32;3],
}

impl Vector3 {
  pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
    return Vector3{e: [x, y, z]};
  }
  pub fn x(&self) -> f32 { return self.e[0]; }
  pub fn y(&self) -> f32 { return self.e[1]; }
  pub fn z(&self) -> f32 { return self.e[2]; }

  pub fn r(&self) -> f32 { return self.e[0]; }
  pub fn g(&self) -> f32 { return self.e[1]; }
  pub fn b(&self) -> f32 { return self.e[2]; }

  pub fn cross(&self, v:Vector3) -> Vector3 { 
      Vector3 { e: [ 
        self.y() * v.z() - self.z() * v.y(),
        self.z() * v.x() - self.x() * v.z(),
        self.x() * v.y() - self.y() * v.x()
     ]}     
  }

  pub fn dot(&self, v:Vector3) -> Vector3 { 
      Vector3 { e: [ 
        self.x() * v.x(),
        self.y() * v.y(),
        self.z() * v.z()
     ]}     
  }

  pub fn squared_length(&self) -> f32 { self.x() * self.x() + self.y() * self.y() + self.z() * self.z() }
  pub fn length(&self) -> f32 { self.squared_length().sqrt() }
  pub fn unit_vector(self) -> Vector3 { self / self.length() }
} 

impl_op_ex!(/= |a: &mut Vector3, b: &f32| {
  a.e[0] /= b;
  a.e[1] /= b;
  a.e[2] /= b;
});

impl_op_ex!(*= |a: &mut Vector3, b: &f32| {
  a.e[0] *= b;
  a.e[1] *= b;
  a.e[2] *= b;
});

impl_op_ex_commutative!(* |a: &Vector3, b: &f32| -> Vector3 {
  Vector3{ e: [a.e[0] * b, a.e[1] * b, a.e[2] * b]}
});

impl_op_ex!(/ |a: &Vector3, b: &f32| -> Vector3 {
  Vector3{ e: [a.e[0] / b, a.e[1] / b, a.e[2] / b]}
});

impl_op_ex!(- |a: &Vector3| -> Vector3 {
  Vector3{ e: [-a.e[0], -a.e[1], -a.e[2]] }
});

impl_op_ex!(+ |a: &Vector3, b: &Vector3| -> Vector3 {
  Vector3{ e: [a.e[0] + b.e[0], a.e[1] + b.e[1], a.e[2] + b.e[2]] }
});

impl_op_ex!(- |a: &Vector3, b: &Vector3| -> Vector3 {
  Vector3{ e: [a.e[0] - b.e[0], a.e[1] - b.e[1], a.e[2] - b.e[2]] }
});

impl_op_ex!(* |a: &Vector3, b: &Vector3| -> Vector3 {
  Vector3{ e: [a.e[0] * b.e[0], a.e[1] * b.e[1], a.e[2] * b.e[2]]}
});

impl_op_ex!(/ |a: &Vector3, b: &Vector3| -> Vector3 {
  Vector3{ e: [a.e[0] / b.e[0], a.e[1] / b.e[1], a.e[2] / b.e[2]]}
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x:Vector3 = Vector3::new(1.0,2.0,3.0);
        let y:Vector3 = Vector3::new(1.0,2.0,3.0);
        assert_eq!(x + y, Vector3::new(2.0,4.0,6.0));
    }

     #[test]
    fn test_scalar_mul() {
        let x:Vector3 = Vector3::new(1.0,2.0,3.0);
        assert_eq!(x * 2.0, Vector3::new(2.0,4.0,6.0));
        assert_eq!(2.0 * x, Vector3::new(2.0,4.0,6.0));
    }

      #[test]
    fn test_assign_div() {
        let mut x:Vector3 = Vector3::new(2.0,4.0,6.0);
        x /= 2.0;
        assert_eq!(x, Vector3::new(1.0,2.0,3.0));
    }
}