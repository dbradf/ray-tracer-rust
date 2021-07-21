use crate::light::Material;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

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

    pub fn intersect<'a>(&self, s: &'a Sphere) -> Intersections<'a> {
        let ray = self.transform(&s.transform.inverse().unwrap());
        let sphere_to_ray = &ray.origin - &Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            Intersections::new(vec![])
        } else {
            Intersections::new(vec![
                Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), s),
                Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), s),
            ])
        }
    }

    pub fn transform(&self, m: &Matrix) -> Self {
        Self::new(&(m * &self.origin), &(m * &self.direction))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub origin: Tuple,
    pub radii: f64,
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            origin: Tuple::point(0.0, 0.0, 0.0),
            radii: 1.0,
            transform: Matrix::identify(),
            material: Material::new(),
        }
    }

    pub fn set_transform(&mut self, transform: &Matrix) {
        self.transform = transform.clone();
    }

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let sphere_inverse = &self.transform.inverse().unwrap();
        let object_point = sphere_inverse * world_point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let world_normal = sphere_inverse.transpose() * object_normal;

        Tuple::vector(world_normal.x, world_normal.y, world_normal.z).normalize()

    }
}

#[derive(Debug, Clone)]
pub struct Computation<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Intersection<'a> {
        Self { t, object }
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

        Computation {
            t: self.t,
            object: self.object,
            point: point.clone(),
            eyev,
            inside,
            normalv,
        }
    }
}

impl<'a> PartialEq for &Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        (self.t == other.t) && (std::ptr::eq(self.object, other.object))
    }
}

#[derive(Debug, Clone)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new(intersections: Vec<Intersection<'a>>) -> Intersections<'a> {
        Self { intersections }
    }

    pub fn count(&self) -> usize {
        self.intersections.len()
    }

    pub fn at<'b>(&self, index: usize) -> &'b Intersection {
        &self.intersections[index]
    }

    pub fn hit<'b>(&self) -> Option<&'b Intersection> {
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
        intersections.intersections.iter().for_each(|i| self.intersections.push(i.clone()));
    }

    pub fn sort(&mut self) {
        self.intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use crate::utils::equal_f64;

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
        let s = Sphere::new();

        let xs = r.intersect(&s);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.at(0).t, 4.0);
        assert_eq!(xs.at(1).t, 6.0);
    }

    #[test]
    fn test_a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(&Tuple::point(0.0, 1.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = r.intersect(&s);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.at(0).t, 5.0);
        assert_eq!(xs.at(1).t, 5.0);
    }

    #[test]
    fn test_a_ray_misses_a_sphere() {
        let r = Ray::new(&Tuple::point(0.0, 2.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = r.intersect(&s);

        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn test_a_ray_originates_inside_a_sphere() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, 0.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = r.intersect(&s);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.at(0).t, -1.0);
        assert_eq!(xs.at(1).t, 1.0);
    }

    #[test]
    fn test_a_ray_originates_behind_a_sphere() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, 5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = r.intersect(&s);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.at(0).t, -6.0);
        assert_eq!(xs.at(1).t, -4.0);
    }

    #[test]
    fn test_an_interestion_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(i.object, &s));
    }

    #[test]
    fn test_aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.count(), 2);
        assert!(std::ptr::eq(xs.at(0).object, &s));
        assert!(std::ptr::eq(xs.at(1).object, &s));
    }

    #[test]
    fn test_intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = r.intersect(&s);

        assert_eq!(xs.count(), 2);
        assert!(std::ptr::eq(xs.at(0).object, &s));
        assert!(std::ptr::eq(xs.at(1).object, &s));
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1.clone(), i2]);

        let i = xs.hit().unwrap();

        assert_eq!(i, &i1);
    }

    #[test]
    fn test_the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);

        let i = xs.hit().unwrap();

        assert_eq!(i, &i2);
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);

        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn test_the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]);

        let i = xs.hit().unwrap();

        assert_eq!(i, &i4);
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

        assert_eq!(s.transform, Matrix::identify());
    }

    #[test]
    fn test_changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2.0, 3.0, 4.0);

        s.set_transform(&t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn test_intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(&Matrix::scaling(2.0, 2.0, 2.0));

        let xs = r.intersect(&s);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.at(0).t, 3.0);
        assert_eq!(xs.at(1).t, 7.0);
    }

    #[test]
    fn test_intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(&Matrix::translation(5.0, 0.0, 0.0));

        let xs = r.intersect(&s);

        assert_eq!(xs.count(), 0);
    }


    // spheres
    #[test]
    fn test_the_normal_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        
        let n = s.normal_at(&Tuple::point(1.0, 0.0, 0.0));

        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_the_normal_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        
        let n = s.normal_at(&Tuple::point(0.0, 1.0, 0.0));

        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_the_normal_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        
        let n = s.normal_at(&Tuple::point(0.0, 0.0, 1.0));

        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_the_normal_sphere_at_a_point_on_a_nonaxial_point() {
        let s = Sphere::new();
        
        let n = s.normal_at(&Tuple::point(3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0));

        assert_eq!(n, Tuple::vector(3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0));
    }

    #[test]
    fn test_the_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(&Tuple::point(3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0));

        assert_eq!(n.clone(), n.normalize());
    }

    #[test]
    fn test_computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(&Matrix::translation(0.0, 1.0, 0.0));

        let n = s.normal_at(&Tuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn test_copmuting_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        let m = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 0.5);
        s.set_transform(&m);

        let n = s.normal_at(&Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));

        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn test_a_sphere_has_a_default_material() {
        let s = Sphere::new();

        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn test_a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;

        s.material = m.clone();

        assert_eq!(s.material, m);
    }

    #[test]
    fn test_precomputing_the_state_of_an_intersection() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computation(&r);

        assert!(equal_f64(comps.t, i.t));
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computation(&r);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn test_the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(&Tuple::point(0.0, 0.0, 0.0), &Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);

        let comps = i.prepare_computation(&r);

        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }
}
