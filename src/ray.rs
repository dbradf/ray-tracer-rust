use crate::matrix::Matrix;
use crate::shapes::Shape;
use crate::tuple::Tuple;
use crate::utils::EPSILON;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: &Tuple, direction: &Tuple) -> Self {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        &self.origin + &(&self.direction * t)
    }

    pub fn intersect(&self, s: Rc<dyn Shape>) -> Intersections {
        let transform = s.get_transform();
        let ray = self.transform(&transform.inverse().unwrap());

        Intersections::new(
            s.intersect(&ray)
                .iter()
                .map(|i| Intersection::new(*i, s.clone()))
                .collect(),
        )
    }

    pub fn transform(&self, m: &Matrix) -> Self {
        Self::new(&(m * &self.origin), &(m * &self.direction))
    }
}

#[derive(Clone)]
pub struct Computation {
    pub t: f64,
    pub object: Rc<dyn Shape>,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

#[derive(Clone, Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Rc<dyn Shape>,
}

impl Intersection {
    pub fn new(t: f64, object: Rc<dyn Shape>) -> Intersection {
        Self {
            t,
            object: object.clone(),
        }
    }

    pub fn prepare_computation(&self, ray: &Ray) -> Computation {
        let point = ray.position(self.t);
        let eyev = -ray.direction.clone();
        let mut normalv = self.object.normal_at(&point);
        let inside = if normalv.dot(&eyev) < 0.0 {
            normalv = -normalv;
            true
        } else {
            false
        };
        let over_point = point.clone() + normalv.clone() * EPSILON;

        Computation {
            t: self.t,
            object: self.object.clone(),
            point: point.clone(),
            eyev,
            inside,
            normalv,
            over_point,
        }
    }
}

impl PartialEq for &Intersection {
    fn eq(&self, other: &Self) -> bool {
        (self.t == other.t) && (std::ptr::eq(self.object.as_ref(), other.object.as_ref()))
    }
}

#[derive(Clone)]
pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Intersections {
        Self { intersections }
    }

    pub fn count(&self) -> usize {
        self.intersections.len()
    }

    pub fn at(&self, index: usize) -> Intersection {
        self.intersections[index].clone()
    }

    pub fn hit(&self) -> Option<Intersection> {
        let mut lowest_index: Option<usize> = None;

        for (i, intersect) in self.intersections.iter().enumerate() {
            if intersect.t > 0.0 {
                if let Some(index) = lowest_index {
                    if intersect.t < self.at(index).t {
                        lowest_index = Some(i);
                    }
                } else {
                    lowest_index = Some(i);
                }
            }
        }

        lowest_index.map(|i| self.at(i))
    }

    pub fn extend(&mut self, intersections: &Self) {
        intersections
            .intersections
            .iter()
            .for_each(|i| self.intersections.push(i.clone()));
    }

    pub fn sort(&mut self) {
        self.intersections
            .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::Sphere;
    use crate::utils::{equal_f64, EPSILON};

    #[test]
    fn test_creating_and_querying_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let ray = Ray::new(&origin, &direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn test_computing_a_point_from_a_distance() {
        let r = Ray::new(&Tuple::point(2.0, 3.0, 4.0), &Tuple::vector(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new());

        let xs = r.intersect(s.clone());

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.at(0).t, 4.0);
        assert_eq!(xs.at(1).t, 6.0);
    }

    #[test]
    fn test_a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(&Tuple::point(0.0, 1.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new());

        let xs = r.intersect(s.clone());

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.at(0).t, 5.0);
        assert_eq!(xs.at(1).t, 5.0);
    }

    #[test]
    fn test_a_ray_misses_a_sphere() {
        let r = Ray::new(&Tuple::point(0.0, 2.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new());

        let xs = r.intersect(s.clone());

        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn test_a_ray_originates_inside_a_sphere() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, 0.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new());

        let xs = r.intersect(s.clone());

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.at(0).t, -1.0);
        assert_eq!(xs.at(1).t, 1.0);
    }

    #[test]
    fn test_a_ray_originates_behind_a_sphere() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, 5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new());

        let xs = r.intersect(s.clone());

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.at(0).t, -6.0);
        assert_eq!(xs.at(1).t, -4.0);
    }

    #[test]
    fn test_an_interestion_encapsulates_t_and_object() {
        let s = Rc::new(Sphere::new());
        let i = Intersection::new(3.5, s.clone());

        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(i.object.as_ref(), s.as_ref()));
    }

    #[test]
    fn test_aggregating_intersections() {
        let s = Rc::new(Sphere::new());
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.count(), 2);
        assert!(std::ptr::eq(xs.at(0).object.as_ref(), s.as_ref()));
        assert!(std::ptr::eq(xs.at(1).object.as_ref(), s.as_ref()));
    }

    #[test]
    fn test_intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new());

        let xs = r.intersect(s.clone());

        assert_eq!(xs.count(), 2);
        assert!(std::ptr::eq(xs.at(0).object.as_ref(), s.as_ref()));
        assert!(std::ptr::eq(xs.at(1).object.as_ref(), s.as_ref()));
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_positive_t() {
        let s = Rc::new(Sphere::new());
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());
        let xs = Intersections::new(vec![i1.clone(), i2]);

        let i = xs.hit().unwrap();

        assert!(&i == &i1);
    }

    #[test]
    fn test_the_hit_when_some_intersections_have_negative_t() {
        let s = Rc::new(Sphere::new());
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s.clone());
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);

        let i = xs.hit().unwrap();

        assert!(&i == &i2);
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_negative_t() {
        let s = Rc::new(Sphere::new());
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s.clone());
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);

        let i = xs.hit();

        assert!(i.is_none());
    }

    #[test]
    fn test_the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Rc::new(Sphere::new());
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s.clone());
        let xs = Intersections::new(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]);

        let i = xs.hit().unwrap();

        assert_eq!(&i, &i4);
    }

    #[test]
    fn test_translating_a_ray() {
        let r = Ray::new(&Tuple::point(1.0, 2.0, 3.0), &Tuple::vector(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);

        let r2 = r.transform(&m);

        assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_scaling_a_ray() {
        let r = Ray::new(&Tuple::point(1.0, 2.0, 3.0), &Tuple::vector(0.0, 1.0, 0.0));
        let m = Matrix::scaling(2.0, 3.0, 4.0);

        let r2 = r.transform(&m);

        assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
    }

    #[test]
    fn test_a_spheres_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.get_transform(), Matrix::identify());
    }

    #[test]
    fn test_changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2.0, 3.0, 4.0);

        s.set_transform(&t);

        assert_eq!(s.get_transform(), t);
    }

    #[test]
    fn test_intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new().with_transform(&Matrix::scaling(2.0, 2.0, 2.0)));

        let xs = r.intersect(s.clone());

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.at(0).t, 3.0);
        assert_eq!(xs.at(1).t, 7.0);
    }

    #[test]
    fn test_intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new().with_transform(&Matrix::translation(5.0, 0.0, 0.0)));

        let xs = r.intersect(s.clone());

        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn test_precomputing_the_state_of_an_intersection() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let shape = Rc::new(Sphere::new());
        let i = Intersection::new(4.0, shape.clone());

        let comps = i.prepare_computation(&r);

        assert!(equal_f64(comps.t, i.t));
        assert!(std::ptr::eq(comps.object.as_ref(), i.object.as_ref()));
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let shape = Rc::new(Sphere::new());
        let i = Intersection::new(4.0, shape.clone());

        let comps = i.prepare_computation(&r);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn test_the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, 0.0), &Tuple::vector(0.0, 0.0, 1.0));
        let shape = Rc::new(Sphere::new());
        let i = Intersection::new(1.0, shape.clone());

        let comps = i.prepare_computation(&r);

        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_the_hit_should_offset_the_point() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let shape = Rc::new(Sphere::new().with_transform(&Matrix::translation(0.0, 0.0, 1.0)));
        let i = Intersection::new(5.0, shape.clone());

        let comps = i.prepare_computation(&r);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
