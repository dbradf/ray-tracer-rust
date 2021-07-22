use ray_tracer::camera::Camera;
use ray_tracer::canvas::Color;
use ray_tracer::light::{Material, PointLight};
use ray_tracer::matrix::Matrix;
use ray_tracer::shapes::{Plane, Shape, Sphere};
use ray_tracer::transformations::view_transform;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;
use std::f64::consts::PI;
use std::rc::Rc;

fn main() {
    let mut floor_m = Material::new();
    floor_m.color = Color::new(0.97, 0.96, 0.94);
    floor_m.specular = 0.0;
    let floor = Rc::new(
        Plane::new()
            .with_material(&floor_m)
            .with_transform(&Matrix::translation(0.0, 0.0, 0.0)),
    );

    let mut middle_m = Material::new();
    middle_m.color = Color::new(0.65, 0.8, 0.83);
    middle_m.diffuse = 0.7;
    middle_m.specular = 0.3;
    let middle = Rc::new(
        Sphere::new()
            .with_material(&middle_m)
            .with_transform(&Matrix::translation(-0.5, 1.0, 0.5)),
    );

    let mut right_m = Material::new();
    right_m.color = Color::new(0.39, 0.59, 0.35);
    right_m.diffuse = 0.7;
    right_m.specular = 0.3;
    let right =
        Rc::new(Sphere::new().with_material(&right_m).with_transform(
            &(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5)),
        ));

    let mut left_m = Material::new();
    left_m.color = Color::new(0.3, 0.3, 0.35);
    left_m.diffuse = 0.7;
    left_m.specular = 0.3;
    let left = Rc::new(Sphere::new().with_material(&left_m).with_transform(
        &(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33)),
    ));

    let mut world = World::new();
    world.light = Some(PointLight::new(
        &Tuple::point(-10.0, 10.0, -10.0),
        &Color::white(),
    ));
    world.objects = vec![floor, middle, right, left];

    let mut camera = Camera::new(1024, 500, PI / 3.0);
    camera.transform = view_transform(
        &Tuple::point(0.0, 1.5, -5.0),
        &Tuple::point(0.0, 1.0, 0.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);
    canvas.save("plane.ppm").unwrap();
}
