use crate::tuple::Tuple;
use crate::matrix::Matrix;

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(&upn);
    let true_up = left.cross(&forward);

    let orientation = Matrix::new(&[
                                  left.x, left.y, left.z, 0.0,
                                  true_up.x, true_up.y, true_up.z, 0.0,
                                  -forward.x, -forward.y, -forward.z, 0.0,
                                  0.0, 0.0, 0.0, 1.0,
    ]);

    orientation * Matrix::translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_transformation_matrix_for_the_default_orientation() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);

        assert_eq!(t, Matrix::identify());
    }

    #[test]
    fn test_a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);

        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn test_the_view_transform_moves_the_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);

        assert_eq!(t, Matrix::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn test_an_arbitrary_view_transformation() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);

        assert_eq!(t, Matrix::new(&[
                                  -0.50709, 0.50709, 0.67612, -2.36643,
                                  0.76772, 0.60609, 0.12122, -2.82843,
                                  -0.35857, 0.59761, -0.71714, 0.0,
                                  0.0, 0.0, 0.0, 1.0,
        ]));
    }
}

