use crate::canvas::{Canvas, Color};
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let (half_width, half_height) = Self::pixel_size(hsize, vsize, field_of_view);
        let pixel_size = (half_width * 2.0) / hsize as f64;
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identify(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    fn pixel_size(hsize: usize, vsize: usize, field_of_view: f64) -> (f64, f64) {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse().unwrap() * Tuple::point(world_x, world_y, -1.0);
        let origin = self.transform.inverse().unwrap() * Tuple::point(0.0, 0.0, 0.0);
        let direction = (&pixel - &origin).normalize();

        Ray::new(&origin, &direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        let n_pixels = self.hsize * self.vsize;
        let pixels: Vec<Color> = (0..n_pixels)
            .into_par_iter()
            .map(|i| {
                let x = i % self.hsize;
                let y = i / self.hsize;

                let ray = self.ray_for_pixel(x, y);
                world.color_at(&ray)
            })
            .collect();

        pixels.iter().enumerate().for_each(|(i, c)| {
            let x = i % self.hsize;
            let y = i / self.hsize;

            image.write_pixel(x, y, c);
        });

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::Color;
    use crate::transformations::view_transform;
    use crate::utils::equal_f64;
    use crate::world::World;
    use std::f64::consts::PI;

    #[test]
    fn test_contructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_eq!(c.field_of_view, field_of_view);
        assert_eq!(c.transform, Matrix::identify());
    }

    #[test]
    fn test_the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert!(equal_f64(c.pixel_size, 0.01));
    }

    #[test]
    fn test_the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!(equal_f64(c.pixel_size, 0.01));
    }

    #[test]
    fn test_contructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_contructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_contructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Matrix::rotation_y(PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Tuple::vector(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn test_rendering_a_world_with_a_camera() {
        let w = World::default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        c.transform = view_transform(&from, &to, &up);

        let image = c.render(&w);

        assert_eq!(image.pixel_at(5, 5), &Color::new(0.38066, 0.47583, 0.2855));
    }
}
