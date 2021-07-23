use crate::canvas::Color;
use crate::shapes::Shape;
use crate::tuple::Tuple;
use crate::pattern::Pattern;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: &Tuple, intensity: &Color) -> Self {
        Self {
            position: position.clone(),
            intensity: intensity.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Arc<dyn Pattern + Sync + Send>>,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,

        }
    }
}

impl std::fmt::Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Material: {{{:?}}}", self.color)
    }
}

impl std::cmp::PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess
    }
}

pub fn lighting(
    material: &Material,
    object: Arc<dyn Shape>,
    light: &PointLight,
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadown: bool,
) -> Color {
    let color = if let Some(pattern) = &material.pattern {
        pattern.at_object(object, point)
    } else {
        material.color
    };
    let effective_color = color * light.intensity;
    let lightv = (light.position.clone() - point.clone()).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = lightv.dot(normalv);
    let (diffuse, specular) = if light_dot_normal < 0.0 {
        (Color::black(), Color::black())
    } else {
        let diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye <= 0.0 {
            (diffuse, Color::black())
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            (diffuse, light.intensity * material.specular * factor)
        }
    };

    if in_shadown {
        ambient
    } else {
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::equal_f64;
    use crate::pattern::StripePattern;
    use crate::shapes::Sphere;

    #[test]
    fn test_a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);

        let light = PointLight::new(&position, &intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn test_the_default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert!(equal_f64(m.ambient, 0.1));
        assert!(equal_f64(m.diffuse, 0.9));
        assert!(equal_f64(m.specular, 0.9));
        assert!(equal_f64(m.shininess, 200.0));
    }

    #[test]
    fn test_lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(&Tuple::point(0.0, 0.0, -10.0), &Color::new(1.0, 1.0, 1.0));

        let result = lighting(&m, Arc::new(Sphere::new()), &light, &position, &eyev, &normalv, false);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(&Tuple::point(0.0, 0.0, -10.0), &Color::new(1.0, 1.0, 1.0));

        let result = lighting(&m, Arc::new(Sphere::new()), &light, &position, &eyev, &normalv, false);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_lighting_with_the_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(&Tuple::point(0.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));

        let result = lighting(&m, Arc::new(Sphere::new()), &light, &position, &eyev, &normalv, false);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn test_lighting_with_the_eye_in_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(&Tuple::point(0.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));

        let result = lighting(&m, Arc::new(Sphere::new()), &light, &position, &eyev, &normalv, false);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn test_lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(&Tuple::point(0.0, 0.0, 10.0), &Color::new(1.0, 1.0, 1.0));

        let result = lighting(&m, Arc::new(Sphere::new()), &light, &position, &eyev, &normalv, false);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_lighting_with_the_surface_in_shadow() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(&Tuple::point(0.0, 0.0, -10.0), &Color::white());
        let in_shadow = true;

        let result = lighting(&m, Arc::new(Sphere::new()), &light, &position, &eyev, &normalv, in_shadow);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_lighting_with_a_pattern_applied() {
        let mut m = Material::new();
        m.pattern = Some(Arc::new(StripePattern::new(&Color::white(), &Color::black())));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(&Tuple::point(0.0, 0.0, -10.0), &Color::white());

        assert_eq!(
            lighting(&m, Arc::new(Sphere::new()), &light, &Tuple::point(0.9, 0.0, 0.0), &eyev, &normalv, false),
            Color::white()
        );
        assert_eq!(
            lighting(&m, Arc::new(Sphere::new()), &light, &Tuple::point(1.1, 0.0, 0.0), &eyev, &normalv, false),
            Color::black()
        );
    }
}
