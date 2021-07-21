use std::cmp::{Eq, PartialEq};

use crate::utils::equal_f64;

#[derive(Debug, PartialEq)]
enum TupleKind {
    Vector,
    Point,
}

#[derive(Debug, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.0)
    }

    fn kind(&self) -> TupleKind {
        if self.w == 0.0 {
            TupleKind::Vector
        } else {
            TupleKind::Point
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Tuple::new(
            self.x / magnitude,
            self.y / magnitude,
            self.z / magnitude,
            self.w / magnitude,
        )
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn reflect(&self, normal: &Tuple) -> Tuple {
        self - &(normal * 2.0 * self.dot(normal))
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equal_f64(self.x, other.x)
            && equal_f64(self.y, other.y)
            && equal_f64(self.z, other.z)
            && equal_f64(self.w, other.w)
    }
}
impl Eq for Tuple {}

impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl std::ops::Add for &Tuple {
    type Output = Tuple;

    fn add(self, rhs: &Tuple) -> Tuple {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl std::ops::Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::new(0.0, 0.0, 0.0, 0.0) - self
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl std::ops::Mul<f64> for &Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert!(equal_f64(a.x, 4.3));
        assert!(equal_f64(a.y, -4.2));
        assert!(equal_f64(a.z, 3.1));
        assert!(equal_f64(a.w, 1.0));
        assert_eq!(a.kind(), TupleKind::Point);
    }

    #[test]
    fn test_a_tuple_with_w_0_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert!(equal_f64(a.x, 4.3));
        assert!(equal_f64(a.y, -4.2));
        assert!(equal_f64(a.z, 3.1));
        assert!(equal_f64(a.w, 0.0));
        assert_eq!(a.kind(), TupleKind::Vector);
    }

    #[test]
    fn test_point_create_a_tuple_with_w_1() {
        let p = Tuple::point(4.0, -4.0, 3.0);

        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn test_vector_create_a_tuple_with_w_0() {
        let v = Tuple::vector(4.0, -4.0, 3.0);

        assert_eq!(v, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn test_adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(&a1 + &a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_subtracting_two_tuples() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_a_vector_from_zero() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_multiplying_a_test_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn test_multiplying_a_test_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_computing_the_magnitude_of_vector_1_0_0() {
        let v0 = Tuple::vector(1.0, 0.0, 0.0);
        let v1 = Tuple::vector(0.0, 1.0, 0.0);
        let v2 = Tuple::vector(0.0, 0.0, 1.0);

        assert_eq!(v0.magnitude(), 1.0);
        assert_eq!(v1.magnitude(), 1.0);
        assert_eq!(v2.magnitude(), 1.0);
    }

    #[test]
    fn test_computing_the_magnitude_of_other_vectors() {
        let v0 = Tuple::vector(1.0, 2.0, 3.0);
        let v1 = Tuple::vector(-1.0, -2.0, -3.0);

        assert!(equal_f64(v0.magnitude(), (14.0_f64).sqrt()));
        assert!(equal_f64(v1.magnitude(), (14.0_f64).sqrt()));
    }

    #[test]
    fn test_normalizing_vectors() {
        let v0 = Tuple::vector(4.0, 0.0, 0.0);
        let v1 = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(v0.normalize(), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(v1.normalize(), Tuple::vector(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn test_magnitude_of_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let norm = v.normalize();

        assert!(equal_f64(norm.magnitude(), 1.0));
    }

    #[test]
    fn test_dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert!(equal_f64(a.dot(&b), 20.0));
    }

    #[test]
    fn test_cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(a.cross(&b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Tuple::vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn test_reflecting_a_vector_approaching_at_45() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);

        let r = v.reflect(&n);

        assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_reflecting_a_vector_off_a_slanted_surface() {
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        let r = v.reflect(&n);

        assert_eq!(r, Tuple::vector(1.0, 0.0, 0.0));
    }
}
