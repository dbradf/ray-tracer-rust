use crate::tuple::Tuple;
use crate::utils::equal_f64;

use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Matrix {
    size: usize,
    elements: Vec<f64>,
}

impl Matrix {
    pub fn new(elements: &[f64]) -> Self {
        let size = (elements.len() as f32).sqrt() as usize;
        Self {
            elements: elements.to_vec(),
            size,
        }
    }

    pub fn identify() -> Self {
        Self::new(&[
                  1.0, 0.0, 0.0, 0.0,
                  0.0, 1.0, 0.0, 0.0,
                  0.0, 0.0, 1.0, 0.0,
                  0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self::new(&[
                  1.0, 0.0, 0.0, x,
                  0.0, 1.0, 0.0, y,
                  0.0, 0.0, 1.0, z,
                  0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self::new(&[
                  x, 0.0, 0.0, 0.0,
                  0.0, y, 0.0, 0.0,
                  0.0, 0.0, z, 0.0,
                  0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn rotation_x(r: f64) -> Self {
        Self::new(&[
                  1.0, 0.0, 0.0, 0.0,
                  0.0, r.cos(), -r.sin(), 0.0,
                  0.0, r.sin(), r.cos(), 0.0,
                  0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn rotation_y(r: f64) -> Self {
        Self::new(&[
                  r.cos(), 0.0, r.sin(), 0.0,
                  0.0, 1.0, 0.0, 0.0,
                  -r.sin(), 0.0, r.cos(), 0.0,
                  0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn rotation_z(r: f64) -> Self {
        Self::new(&[
                  r.cos(), -r.sin(), 0.0, 0.0,
                  r.sin(), r.cos(), 0.0, 0.0,
                  0.0, 0.0, 1.0, 0.0,
                  0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Self {
        Self::new(&[
                  1.0, x_y, x_z, 0.0,
                  y_x, 1.0, y_z, 0.0,
                  z_x, z_y, 1.0, 0.0,
                  0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn at(&self, y: usize, x: usize) -> f64 {
        let index = self.index(x, y);
        self.elements[index]
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    fn mul_item(&self, rhs: &Matrix, row: usize, col: usize) -> f64 {
        (0..self.size)
            .into_iter()
            .map(|i| self.at(row, i) * rhs.at(i, col))
            .sum()
    }

    pub fn transpose(&self) -> Matrix {
        let elements: Vec<f64> = (0..self.size * self.size).into_iter().map(|index| {
            let row = index / self.size;
            let col = index % self.size;

            self.at(col, row)
        }).collect();

        Matrix::new(&elements)
    }

    pub fn determinant(&self) -> f64 {
        if self.size == 2 {
            self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0)
        } else {
            (0..self.size).into_iter().map(|i| self.at(0, i) * self.cofactor(0, i)).sum()
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let elements: Vec<f64> = (0..self.size*self.size).into_iter().map(|index| {
            let r = index / self.size;
            let c = index % self.size;

            if r == row || c == col {
                None
            } else {
                Some(self.at(r, c))
            }
        }).filter_map(|x| x).collect();

        Matrix::new(&elements)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn is_invertible(&self) -> bool {
        !equal_f64(self.determinant(), 0.0)
    }

    pub fn inverse(&self) -> Option<Matrix> {
        let det = self.determinant();
        if equal_f64(det, 0.0) {
            return None
        }

        let elements: Vec<f64> = (0..self.size * self.size).into_iter().map(|index| {
            let row = index / self.size;
            let col = index % self.size;

            let c = self.cofactor(col, row);

            c / det
        }).collect();

        Some(Matrix::new(&elements))
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.elements.iter().zip(&other.elements).all(|(a, b)| equal_f64(*a, *b))
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let elements: Vec<f64> = (0..self.size * self.size)
            .into_iter()
            .map(|index| {
                let row = index / self.size;
                let col = index % self.size;

                self.mul_item(&rhs, row, col)
            })
            .collect();
        Self::new(&elements)
    }
}

impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.at(0, 0) * rhs.x
                + self.at(0, 1) * rhs.y
                + self.at(0, 2) * rhs.z
                + self.at(0, 3) * rhs.w,
            self.at(1, 0) * rhs.x
                + self.at(1, 1) * rhs.y
                + self.at(1, 2) * rhs.z
                + self.at(1, 3) * rhs.w,
            self.at(2, 0) * rhs.x
                + self.at(2, 1) * rhs.y
                + self.at(2, 2) * rhs.z
                + self.at(2, 3) * rhs.w,
            self.at(3, 0) * rhs.x
                + self.at(3, 1) * rhs.y
                + self.at(3, 2) * rhs.z
                + self.at(3, 3) * rhs.w,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructing_and_inspecting_a_4_x_4_matrix() {
        let m = Matrix::new(&[
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ]);

        assert!(equal_f64(m.at(0, 0), 1.0));
        assert!(equal_f64(m.at(0, 3), 4.0));
        assert!(equal_f64(m.at(1, 0), 5.5));
        assert!(equal_f64(m.at(1, 2), 7.5));
        assert!(equal_f64(m.at(2, 2), 11.0));
        assert!(equal_f64(m.at(3, 0), 13.5));
        assert!(equal_f64(m.at(3, 2), 15.5));
    }

    #[test]
    fn test_constructing_a_2x2_matrix() {
        let m = Matrix::new(&[-3.0, 5.0, 1.0, -2.0]);

        assert!(equal_f64(m.at(0, 0), -3.0));
        assert!(equal_f64(m.at(0, 1), 5.0));
        assert!(equal_f64(m.at(1, 0), 1.0));
        assert!(equal_f64(m.at(1, 1), -2.0));
    }

    #[test]
    fn test_constructing_a_3x3_matrix() {
        let m = Matrix::new(&[-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert!(equal_f64(m.at(0, 0), -3.0));
        assert!(equal_f64(m.at(1, 1), -2.0));
        assert!(equal_f64(m.at(2, 2), 1.0));
    }

    #[test]
    fn test_matrix_equality_with_identical_matrices() {
        let a = Matrix::new(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matrix::new(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn test_matrix_equality_with_different_matrices() {
        let a = Matrix::new(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matrix::new(&[
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ]);

        assert_ne!(a, b);
    }

    #[test]
    fn test_multiplying_two_matrices() {
        let a = Matrix::new(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matrix::new(&[
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ]);

        assert_eq!(
            a * b,
            Matrix::new(&[
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ])
        );
    }

    #[test]
    fn test_multiplying_a_matrix_by_a_tuple() {
        let a = Matrix::new(&[
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn test_multiplying_by_identity_matrix() {
        let a = Matrix::new(&[
                            0.0, 1.0, 2.0, 4.0,
                            1.0, 2.0, 4.8, 8.0,
                            2.0, 4.0, 8.0, 16.0,
                            4.0, 8.0, 16.0, 32.0,
        ]);
        let id = Matrix::identify();

        assert_eq!(a.clone() * id, a);
    }

    #[test]
    fn test_multiplying_identify_matrix_by_tuple() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let id = Matrix::identify();

        assert_eq!(id * a.clone(), a);
    }

    #[test]
    fn test_transposing_a_matrix() {
        let a = Matrix::new(&[
                            0.0, 9.0, 3.0, 0.0,
                            9.0, 8.0, 0.0, 8.0,
                            1.0, 8.0, 5.0, 3.0,
                            0.0, 0.0, 5.0, 8.0,
        ]);

        assert_eq!(a.transpose(), Matrix::new(&[
                                              0.0, 9.0, 1.0, 0.0,
                                              9.0, 8.0, 8.0, 0.0,
                                              3.0, 0.0, 5.0, 5.0,
                                              0.0, 8.0, 3.0, 8.0,
        ]));
    }

    #[test]
    fn test_transposing_identify_matrix() {
        let a = Matrix::identify();

        assert_eq!(a.transpose(), Matrix::identify());
    }

    #[test]
    fn test_calc_the_determinate_of_a_2x2_matrix() {
        let a = Matrix::new(&[1.0, 5.0, -3.0, 2.0]);

        assert!(equal_f64(a.determinant(), 17.0));
    }

    #[test]
    fn test_submatrix_of_a_3x3_matrix_should_be_a_2x2_matrix() {
        let a = Matrix::new(&[
                            1.0, 5.0, 0.0,
                            -3.0, 2.0, 7.0,
                            0.0, 6.0, -3.0,
        ]);

        assert_eq!(a.submatrix(0, 2), Matrix::new(&[-3.0, 2.0, 0.0, 6.0]));
    }
    
    #[test]
    fn test_submatrix_of_a_4x4_matrix_should_be_a_3x3_matrix() {
        let a = Matrix::new(&[
                            -6.0, 1.0, 1.0, 6.0,
                            -8.0, 5.0, 8.0, 6.0,
                            -1.0, 0.0, 8.0, 2.0,
                            -7.0, 1.0, -1.0, 1.0,
        ]);

        assert_eq!(a.submatrix(2, 1), Matrix::new(&[
                                                  -6.0, 1.0, 6.0,
                                                  -8.0, 8.0, 6.0,
                                                  -7.0, -1.0, 1.0,
        ]));
    }

    #[test]
    fn test_calculating_minor_of_3x3_matrix() {
        let a = Matrix::new(&[
                            3.0, 5.0, 0.0,
                            2.0, -1.0, -7.0,
                            6.0, -1.0, 5.0,
        ]);
        let b = a.submatrix(1, 0);

        assert!(equal_f64(b.determinant(), 25.0));
        assert!(equal_f64(a.minor(1, 0), 25.0));
    }

    #[test]
    fn test_calc_cofactor_of_a_3x3_matrix() {
        let a = Matrix::new(&[
                            3.0, 5.0, 0.0,
                            2.0, -1.0, -7.0,
                            6.0, -1.0, 5.0,
        ]);

        assert!(equal_f64(a.minor(0, 0), -12.0));
        assert!(equal_f64(a.cofactor(0, 0), -12.0));
        assert!(equal_f64(a.minor(1, 0), 25.0));
        assert!(equal_f64(a.cofactor(1, 0), -25.0));
    }

    #[test]
    fn test_calc_determinant_of_a_3x3_matrix() {
        let a = Matrix::new(&[
                            1.0, 2.0, 6.0,
                            -5.0, 8.0, -4.0,
                            2.0, 6.0, 4.0,
        ]);

        assert!(equal_f64(a.cofactor(0, 0), 56.0));
        assert!(equal_f64(a.cofactor(0, 1), 12.0));
        assert!(equal_f64(a.cofactor(0, 2), -46.0));
        assert!(equal_f64(a.determinant(), -196.0));
    }

    #[test]
    fn test_calc_determinant_of_a_4x4_matrix() {
        let a = Matrix::new(&[
                            -2.0, -8.0, 3.0, 5.0,
                            -3.0, 1.0, 7.0, 3.0,
                            1.0, 2.0, -9.0, 6.0,
                            -6.0, 7.0, 7.0, -9.0,
        ]);

        assert!(equal_f64(a.cofactor(0, 0), 690.0));
        assert!(equal_f64(a.cofactor(0, 1), 447.0));
        assert!(equal_f64(a.cofactor(0, 2), 210.0));
        assert!(equal_f64(a.cofactor(0, 3), 51.0));
        assert!(equal_f64(a.determinant(), -4071.0));
    }

    #[test]
    fn test_an_invertible_matrix_for_invertibility() {
        let a = Matrix::new(&[
                            6.0, 4.0, 4.0, 4.0,
                            5.0, 5.0, 7.0, 6.0,
                            4.0, -9.0, 3.0, -7.0,
                            9.0, 1.0, 7.0, -6.0,
        ]);

        assert!(equal_f64(a.determinant(), -2120.0));
        assert!(a.is_invertible());
    }

    #[test]
    fn test_a_noninvertible_matrix_for_invertibility() {
        let a = Matrix::new(&[
                            -4.0, 2.0, -2.0, -3.0,
                            9.0, 6.0, 2.0, 6.0,
                            0.0, -5.0, 1.0, -5.0,
                            0.0, 0.0, 0.0, 0.0,
        ]);

        assert!(equal_f64(a.determinant(), 0.0));
        assert!(!a.is_invertible());
    }

    #[test]
    fn test_inverting_a_matrix() {
        let a = Matrix::new(&[
                            -5.0, 2.0, 6.0, -8.0,
                            1.0, -5.0, 1.0, 8.0,
                            7.0, 7.0, -6.0, -7.0,
                            1.0, -3.0, 7.0, 4.0,
        ]);
        let b = a.inverse().unwrap();

        assert!(equal_f64(a.determinant(), 532.0));
        assert!(equal_f64(a.cofactor(2, 3), -160.0));
        assert!(equal_f64(b.at(3, 2), -160.0/532.0));
        assert!(equal_f64(a.cofactor(3, 2), 105.0));
        assert!(equal_f64(b.at(2, 3), 105.0/532.0));
        assert_eq!(b, Matrix::new(&[
                                  0.21805, 0.45113, 0.24060, -0.04511,
                                  -0.80827, -1.45677, -0.44361, 0.52068,
                                  -0.07895, -0.22368, -0.05263, 0.19737,
                                  -0.52256, -0.81391, -0.30075, 0.30639,
        ]));
    }

    #[test]
    fn test_inverting_another_matrix() {
        let a = Matrix::new(&[
                            8.0, -5.0, 9.0, 2.0,
                            7.0, 5.0, 6.0, 1.0,
                            -6.0, 0.0, 9.0, 6.0,
                            -3.0, 0.0, -9.0, -4.0,
        ]);

        assert_eq!(a.inverse(), Some(Matrix::new(&[
                                            -0.15385, -0.15385, -0.28205, -0.53846,
                                            -0.07692, 0.12308, 0.02564, 0.03077,
                                            0.35897, 0.35897, 0.43590, 0.92308,
                                            -0.69231, -0.69231, -0.76923, -1.92308,
        ])));
    }

    #[test]
    fn test_inverting_a_third_matrix() {
        let a = Matrix::new(&[
                            9.0, 3.0, 0.0, 9.0,
                            -5.0, -2.0, -6.0, -3.0,
                            -4.0, 9.0, 6.0, 4.0,
                            -7.0, 6.0, 6.0, 2.0,
        ]);

        assert_eq!(a.inverse(), Some(Matrix::new(&[
                                                 -0.04074, -0.07778, 0.14444, -0.22222,
                                                 -0.07778, 0.03333, 0.36667, -0.33333,
                                                 -0.02901, -0.14630, -0.10926, 0.12963,
                                                 0.17778, 0.06667, -0.26667, 0.33333,
        ])));
    }

    #[test]
    fn test_multiplying_a_product_by_inverse() {
        let a = Matrix::new(&[
                            3.0, -9.0, 7.0, 3.0,
                            3.0, -8.0, 2.0, -9.0,
                            -4.0, 4.0, 4.0, 1.0,
                            -6.0, 5.0, -1.0, 1.0,
        ]);
        let b = Matrix::new(&[
                            8.0, 2.0, 2.0, 2.0,
                            3.0, -1.0, 7.0, 0.0,
                            7.0, 0.0, 5.0, 4.0,
                            6.0, -2.0, 0.0, 5.0,
        ]);

        let c = a.clone() * b.clone();
        
        assert_eq!(c * b.inverse().unwrap(), a);
    }

    #[test]
    fn test_multiplying_by_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform.inverse().unwrap() * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_translation_does_not_affect_vectors() {
        let transform = Matrix::translation(5.0, -3.0, -2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v.clone(), v);
    }

    #[test]
    fn test_a_scaling_matrix_applied_to_a_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_a_scaling_matrix_applied_to_a_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transform.inverse().unwrap() * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);

        assert_eq!(half_quarter * p.clone(), Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0));
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_inverse_of_an_x_rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);

        assert_eq!(half_quarter.inverse().unwrap() * p, Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);

        assert_eq!(half_quarter * p.clone(), Tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0));
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);

        assert_eq!(half_quarter * p.clone(), Tuple::point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0));
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn test_individual_transformation_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn test_chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let t = c * b * a;

        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }

}
