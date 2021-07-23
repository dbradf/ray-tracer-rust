use crate::canvas::Color;
use crate::matrix::Matrix;
use crate::shapes::Shape;
use crate::tuple::Tuple;
use crate::utils::equal_f64;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Pattern {
    fn get_transform(&self) -> Matrix;
    fn pattern_at(&self, point: &Tuple) -> Color;

    fn at_object(&self, object: Arc<dyn Shape>, point: &Tuple) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * point;
        let pattern_point = self.get_transform().inverse().unwrap() * object_point;

        self.pattern_at(&pattern_point)
    }
}

impl Debug for dyn Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pattern transform: {{{:?}}}", self.get_transform())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl StripePattern {
    pub fn new(color_a: &Color, color_b: &Color) -> Self {
        Self {
            a: color_a.clone(),
            b: color_b.clone(),
            transform: Matrix::identify(),
        }
    }

    pub fn with_transform(&self, transform: &Matrix) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            transform: transform.clone(),
        }
    }
}

impl Pattern for StripePattern {
    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn pattern_at(&self, point: &Tuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradientPattern {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl GradientPattern {
    pub fn new(color_a: &Color, color_b: &Color) -> Self {
        Self {
            a: color_a.clone(),
            b: color_b.clone(),
            transform: Matrix::identify(),
        }
    }

    pub fn with_transform(&self, transform: &Matrix) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            transform: transform.clone(),
        }
    }
}

impl Pattern for GradientPattern {
    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn pattern_at(&self, point: &Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        self.a + distance * fraction
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RingPattern {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl RingPattern {
    pub fn new(color_a: &Color, color_b: &Color) -> Self {
        Self {
            a: color_a.clone(),
            b: color_b.clone(),
            transform: Matrix::identify(),
        }
    }

    pub fn with_transform(&self, transform: &Matrix) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            transform: transform.clone(),
        }
    }
}

impl Pattern for RingPattern {
    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn pattern_at(&self, point: &Tuple) -> Color {
        if equal_f64(
            (point.x * point.x + point.z * point.z).sqrt().floor() % 2.0,
            0.0,
        ) {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckersPattern {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl CheckersPattern {
    pub fn new(color_a: &Color, color_b: &Color) -> Self {
        Self {
            a: color_a.clone(),
            b: color_b.clone(),
            transform: Matrix::identify(),
        }
    }

    pub fn with_transform(&self, transform: &Matrix) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            transform: transform.clone(),
        }
    }
}

impl Pattern for CheckersPattern {
    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn pattern_at(&self, point: &Tuple) -> Color {
        if equal_f64(
            (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0,
            0.0,
        ) {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, shapes::Sphere};

    use super::*;

    #[test]
    fn test_creating_a_stripe_pattern() {
        let pattern = StripePattern::new(&Color::white(), &Color::black());

        assert_eq!(pattern.a, Color::white());
        assert_eq!(pattern.b, Color::black());
    }

    #[test]
    fn test_a_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(&Color::white(), &Color::black());

        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 1.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 2.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn test_a_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(&Color::white(), &Color::black());

        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 1.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 2.0)),
            Color::white()
        );
    }

    #[test]
    fn test_a_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(&Color::white(), &Color::black());

        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.9, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(-0.1, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(-1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(-1.1, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn test_stripes_with_an_object_transformation() {
        let object = Sphere::new().with_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let pattern = StripePattern::new(&Color::white(), &Color::black());

        let c = pattern.at_object(Arc::new(object), &Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        let object = Sphere::new();
        let pattern = StripePattern::new(&Color::white(), &Color::black())
            .with_transform(&Matrix::scaling(2.0, 2.0, 2.0));

        let c = pattern.at_object(Arc::new(object), &Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Sphere::new().with_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let pattern = StripePattern::new(&Color::white(), &Color::black())
            .with_transform(&Matrix::translation(0.5, 0.0, 0.0));

        let c = pattern.at_object(Arc::new(object), &Tuple::point(2.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn test_a_gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::new(&Color::white(), &Color::black());

        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn test_a_ring_should_extend_in_both_x_and_z() {
        let pattern = RingPattern::new(&Color::white(), &Color::black());

        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 1.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.708, 0.0, 0.708)),
            Color::black()
        );
    }

    #[test]
    fn test_checkers_should_repeat_in_x() {
        let pattern = CheckersPattern::new(&Color::white(), &Color::black());

        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.99, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(1.01, 0.0, 0.0)),
            Color::black()
        );
    }

    #[test]
    fn test_checkers_should_repeat_in_y() {
        let pattern = CheckersPattern::new(&Color::white(), &Color::black());

        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.99, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 1.01, 0.0)),
            Color::black()
        );
    }

    #[test]
    fn test_checkers_should_repeat_in_z() {
        let pattern = CheckersPattern::new(&Color::white(), &Color::black());

        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.99)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Tuple::point(0.0, 0.0, 1.01)),
            Color::black()
        );
    }
}
