use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::matrix::Matrix;
use crate::light::Material;

pub trait Shape {
    fn get_transform(&self) -> Matrix;
    fn set_transform(&mut self, transform: &Matrix);

    fn get_material(&self) -> Material;
    fn set_material(&mut self, material: &Material);

    fn intersect(&self, ray: &Ray) -> Vec<f64>;

    fn local_normal_at(&self, local_point: &Tuple) -> Tuple;
    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let transform = self.get_transform();
        let shape_inverse = &transform.inverse().unwrap();
        let local_point = shape_inverse * world_point;
        let local_normal = self.local_normal_at(&local_point);
        let world_normal = shape_inverse.transpose() * local_normal;

        Tuple::vector(world_normal.x, world_normal.y, world_normal.z).normalize()

    }
}

#[derive(Debug, Clone)]
struct TestShape {
    transform: Matrix,
    material: Material,
}

impl TestShape {
    fn new() -> Self {
        TestShape {
            transform: Matrix::identify(),
            material: Material::new(),
        }
    }
}

impl Shape for TestShape {
    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn set_transform(&mut self, transform: &Matrix) {
        self.transform = transform.clone()
    }

    fn get_material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, material: &Material) {
        self.material = material.clone();
    }

    fn local_normal_at(&self, local_point: &Tuple) -> Tuple {
        Tuple::vector(local_point.x, local_point.y, local_point.z)
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub origin: Tuple,
    pub radii: f64,
    transform: Matrix,
    material: Material,
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
}

impl Shape for Sphere {
    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn set_transform(&mut self, transform: &Matrix) {
        self.transform = transform.clone();
    }

    fn get_material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, material: &Material) {
        self.material = material.clone();
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = &ray.origin - &Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            vec![]
        } else {
            vec![
                (-b - discriminant.sqrt()) / (2.0 * a),
                (-b + discriminant.sqrt()) / (2.0 * a),
            ]
        }
    }

    fn local_normal_at(&self, local_point: &Tuple) -> Tuple {
        local_point - &Tuple::point(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // Shapes
    #[test]
    fn test_the_default_transformation() {
        let s = TestShape::new();

        assert_eq!(s.get_transform(), Matrix::identify());
    }

    #[test]
    fn test_assigning_a_transform() {
        let mut s = TestShape::new();
        s.set_transform(&Matrix::translation(2.0, 3.0, 4.0));

        assert_eq!(s.get_transform(), Matrix::translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_the_default_material() {
        let s = TestShape::new();
        
        assert_eq!(s.get_material(), Material::new());
    }

    #[test]
    fn test_assign_a_material() {
        let mut s = TestShape::new();
        let mut m = Material::new();
        m.ambient = 1.0;

        s.set_material(&m);

        assert_eq!(s.get_material(), m);
    }

    #[test]
    fn test_computing_the_normal_on_a_translated_shape() {
        let mut s = TestShape::new();
        s.set_transform(&Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Tuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn test_computing_the_normal_on_a_transformed_shape() {
        let mut s = TestShape::new();
        let m = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0);
        s.set_transform(&m);
        let n = s.normal_at(&Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));

        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
    }

    // Spheres
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
}

