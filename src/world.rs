use crate::canvas::Color;
use crate::light::{lighting, Material, PointLight};
use crate::matrix::Matrix;
use crate::ray::{Computation, Intersections, Ray};
use crate::shapes::{Shape, Sphere};
use crate::tuple::Tuple;
use std::sync::Arc;

pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Arc<dyn Shape + Send + Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            light: None,
            objects: vec![],
        }
    }

    pub fn default_world() -> Self {
        let mut m = Material::new();
        m.color = Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        Self::default_world_with_material(&m)
    }

    pub fn default_world_with_material(material: &Material) -> Self {
        let light = PointLight::new(&Tuple::point(-10.0, 10.0, -10.0), &Color::white());
        let s1 = Arc::new(Sphere::new().with_material(material));
        let s2 = Arc::new(Sphere::new().with_transform(&Matrix::scaling(0.5, 0.5, 0.5)));

        Self {
            light: Some(light),
            objects: vec![s1, s2],
        }
    }

    pub fn contains(&self, object: Arc<dyn Shape + Send + Sync>) -> bool {
        for o in &self.objects {
            if Arc::ptr_eq(o, &object) {
                return true;
            }
        }
        false
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        if self.objects.len() <= 0 {
            return Intersections::new(vec![]);
        }

        let mut intersections = ray.intersect(self.objects[0].clone());
        for o in &self.objects[1..] {
            intersections.extend(&ray.intersect(o.clone()));
        }

        intersections.sort();
        intersections
    }

    pub fn shade_hit(&self, comps: &Computation) -> Color {
        if let Some(light) = &self.light {
            let is_shadowed = self.is_shadowed(&comps.over_point);
            lighting(
                &comps.object.get_material(),
                comps.object.clone(),
                light,
                &comps.point,
                &comps.eyev,
                &comps.normalv,
                is_shadowed,
            )
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

    pub fn is_shadowed(&self, point: &Tuple) -> bool {
        if let Some(light) = &self.light {
            let v = &light.position - point;
            let distance = v.magnitude();
            let direction = v.normalize();

            let r = Ray::new(&point, &direction);
            let intersections = self.intersect(&r);

            if let Some(h) = intersections.hit() {
                h.t < distance
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Intersection;

    #[test]
    fn test_creating_a_world() {
        let w = World::new();

        assert!(w.light.is_none());
        assert_eq!(w.objects.len(), 0);
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
        let shape = w.objects[0].clone();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computation(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shading_an_intersection_from_the_inside() {
        let mut w = World::default_world();
        w.light = Some(PointLight::new(
            &Tuple::point(0.0, 0.25, 0.0),
            &Color::white(),
        ));
        let r = Ray::new(&Tuple::point(0.0, 0.0, 0.0), &Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
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
    fn test_there_is_no_shadown_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default_world();
        let p = Tuple::point(0.0, 10.0, 0.0);

        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn test_the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default_world();
        let p = Tuple::point(10.0, -10.0, 10.0);

        assert_eq!(w.is_shadowed(&p), true);
    }

    #[test]
    fn test_there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default_world();
        let p = Tuple::point(-20.0, 20.0, -20.0);

        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn test_there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default_world();
        let p = Tuple::point(-2.0, 2.0, -2.0);

        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn test_shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::default_world();
        w.light = Some(PointLight::new(
            &Tuple::point(0.0, 0.0, -10.0),
            &Color::white(),
        ));
        let s1 = Arc::new(Sphere::new());
        let s2 = Arc::new(Sphere::new().with_transform(&Matrix::translation(0.0, 0.0, 10.0)));
        w.objects = vec![s1.clone(), s2.clone()];
        let r = Ray::new(&Tuple::point(0.0, 0.0, 5.0), &Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, s2);

        let comps = i.prepare_computation(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
