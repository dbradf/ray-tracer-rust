use crate::tuple::Tuple;
use crate::light::{PointLight, Material, lighting};
use crate::canvas::Color;
use crate::ray::{Sphere, Ray, Intersections, Intersection, Computation};
use crate::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        Self {
            light: None,
            objects: vec![],
        }
    }

    pub fn default_world() -> Self {
        let light = PointLight::new(&Tuple::point(-10.0, 10.0, -10.0), &Color::white());
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));

        Self {
            light: Some(light),
            objects: vec![s1, s2],
        }
    }

    pub fn contains(&self, object: &Sphere) -> bool {
        self.objects.contains(object)
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        if self.objects.len() <= 0 {
            return Intersections::new(vec![]);
        }

        let mut intersections = ray.intersect(&self.objects[0]);
        for o in &self.objects[1..] {
            intersections.extend(&ray.intersect(&o));
        }

        intersections.sort();
        intersections
    }

    pub fn shade_hit(&self, comps: &Computation) -> Color {
        if let Some(light) = &self.light {
            lighting(&comps.object.material, light, &comps.point, &comps.eyev, &comps.normalv)
        } else {
            Color::black()
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);
        if let Some(hit) = intersections.hit() {
            let comps = hit.prepare_computation(ray);
            self.shade_hit(&comps)
        } else {
            Color::black()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_a_world() {
        let w = World::new();

        assert!(w.light.is_none());
        assert_eq!(w.objects.len(), 0);
    }

    #[test]
    fn test_the_default_world() {
        let light = PointLight::new(&Tuple::point(-10.0, 10.0, -10.0), &Color::white());
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));

        let w = World::default_world();

        assert_eq!(w.light, Some(light));
        assert!(w.contains(&s1));
        assert!(w.contains(&s2));
    }

    #[test]
    fn test_intersect_a_world_with_a_ray() {
        let w = World::default_world();
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);

        assert_eq!(xs.count(), 4);
        assert_eq!(xs.at(0).t, 4.0);
        assert_eq!(xs.at(1).t, 4.5);
        assert_eq!(xs.at(2).t, 5.5);
        assert_eq!(xs.at(3).t, 6.0);
    }

    #[test]
    fn test_shading_an_intersection() {
        let w = World::default_world();
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computation(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shading_an_intersection_from_the_inside() {
        let mut w = World::default_world();
        w.light = Some(PointLight::new(&Tuple::point(0.0, 0.25, 0.0), &Color::white()));
        let r = Ray::new(&Tuple::point(0.0, 0.0, 0.0), &Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);

        let comps = i.prepare_computation(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_the_color_when_a_ray_misses() {
        let w = World::default_world();
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 1.0, 0.0));

        let c = w.color_at(&r);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn test_the_color_when_a_ray_hits() {
        let w = World::default_world();
        let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));

        let c = w.color_at(&r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default_world();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let r = Ray::new(&Tuple::point(0.0, 0.0, 0.75), &Tuple::vector(0.0, 0.0, -1.0));

        let c = w.color_at(&r);

        assert_eq!(c, w.objects[1].material.color);
    }
}
