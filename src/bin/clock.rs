use ray_tracer::canvas::{Canvas, Color};
use ray_tracer::tuple::Tuple;
use ray_tracer::matrix::Matrix;
use std::f64::consts::PI;

use std::fs::File;
use std::io::Write;

fn main() {
    let mut canvas = Canvas::new(600, 600);
    let color = Color::new(128.0, 0.0, 128.0);
    let origin = Tuple::point(0.0, 0.0, 0.0);

    let points: Vec<Tuple> = (0..12).into_iter().map(|i| {
        let transform = Matrix::rotation_y(i as f64 * PI/6.0) * Matrix::translation(0.0, 0.0, 1.0);
        transform * origin.clone()
    }).collect();

    for p in points {
        write_dot(&mut canvas, p.x, p.z, &color);
        println!("{:?}", p);
    }

    let ppm_contents = canvas.to_ppm();
    let mut file = File::create("clock.ppm").unwrap();
    write!(&mut file, "{}", ppm_contents).unwrap();
}


fn write_dot(c: &mut Canvas, x: f64, y: f64, color: &Color) {
    let x_pixel = translate_pixel(x, c.width, 10);
    let y_pixel = translate_pixel(y, c.height, 10);

    println!("{}, {}", x_pixel, y_pixel);

    for j in y_pixel - 2..y_pixel + 2 {
        for i in x_pixel - 2..x_pixel + 2 {
            c.write_pixel(i, j, color);
        }
    }
}

fn translate_pixel(x: f64, width: usize, padding: usize) -> usize {
    let mid_point = width as f64 / 2.0;
    (mid_point + (mid_point - padding as f64) * x) as usize
}

