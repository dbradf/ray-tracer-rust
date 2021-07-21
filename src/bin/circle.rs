use ray_tracer::canvas::{Canvas, Color};
use ray_tracer::light::{Material, PointLight, lighting};
use ray_tracer::matrix::Matrix;
use ray_tracer::ray::Ray;
use ray_tracer::shapes::{Sphere, Shape};
use ray_tracer::tuple::Tuple;
use std::f64::consts::PI;

fn main() {
    let canvas_pixels = 100;
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let mut shape = Sphere::new();
    shape.set_transform(&(Matrix::rotation_z(PI / 4.0) * Matrix::scaling(0.5, 1.0, 1.0)));
    // shape.set_transform(
      //  &(Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * Matrix::scaling(0.5, 1.0, 1.0)),
    //);
    let mut shape_m = Material::new();
    shape_m.color = Color::new(1.0, 0.2, 1.0);
    shape.set_material(&shape_m);

    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Color::white();
    let light = PointLight::new(&light_position, &light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);

            let r = Ray::new(&ray_origin, &(position - ray_origin.clone()).normalize());
            let xs = r.intersect(&shape);

            if let Some(hit) = xs.hit() {
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(&point);
                let eye = -r.direction;
                let color = lighting(&hit.object.get_material(), &light, &point, &eye, &normal, false);

                canvas.write_pixel(x, y, &color);
            }
        }
    }

    canvas.save("circle.ppm").unwrap();
}
