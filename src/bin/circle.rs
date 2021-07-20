use ray_tracer::canvas::{Canvas, Color};
use ray_tracer::matrix::Matrix;
use ray_tracer::ray::{Ray, Sphere};
use ray_tracer::tuple::Tuple;
use std::f64::consts::PI;

fn main() {
    let canvas_pixels = 100;
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);

    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let mut shape = Sphere::new();
    // shape.set_transform(&(Matrix::rotation_z(PI / 4.0) * Matrix::scaling(0.5, 1.0, 1.0)));
    shape.set_transform(
        &(Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * Matrix::scaling(0.5, 1.0, 1.0)),
    );

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);

            let r = Ray::new(&ray_origin, &(position - ray_origin.clone()).normalize());
            let xs = r.intersect(&shape);

            if xs.hit().is_some() {
                canvas.write_pixel(x, y, &color);
            }
        }
    }

    canvas.save("circle.ppm").unwrap();
}
