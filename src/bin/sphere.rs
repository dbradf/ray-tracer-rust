use ray_tracer::ray::Sphere;
use ray_tracer::matrix::Matrix;
use ray_tracer::light::{Material, PointLight};
use ray_tracer::canvas::Color;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;
use ray_tracer::camera::Camera;
use ray_tracer::transformations::view_transform;
use std::f64::consts::PI;

fn main() {
    let mut floor = Sphere::new();
    floor.transform = Matrix::scaling(10.0, 0.01, 10.0);
    floor.material = Material::new();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::new();
    left_wall.transform = Matrix::translation(0.0, 0.0, 5.0) * Matrix::rotation_y(-PI / 4.0) * Matrix::rotation_x(PI / 2.0) * Matrix::scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material.clone();

    let mut right_wall = Sphere::new();
    right_wall.transform = Matrix::translation(0.0, 0.0, 5.0) * Matrix::rotation_y(PI / 4.0) * Matrix::rotation_x(PI / 2.0) * Matrix::scaling(10.0, 0.01, 10.0);
    right_wall.material = floor.material.clone();

    let mut middle = Sphere::new();
    middle.transform = Matrix::translation(-0.5, 1.0, 0.5);
    middle.material = Material::new();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.transform = Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5);
    right.material = Material::new();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33);
    left.material = Material::new();
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    world.light = Some(PointLight::new(&Tuple::point(-10.0, 10.0, -10.0), &Color::white()));
    world.objects = vec![
        floor,
        left_wall,
        right_wall,
        middle,
        right,
        left,
    ];

    let mut camera = Camera::new(1024, 500, PI / 3.0);
    camera.transform = view_transform(&Tuple::point(0.0, 1.5, -5.0), &Tuple::point(0.0, 1.0, 0.0), &Tuple::vector(0.0, 1.0, 0.0));

    let canvas = camera.render(&world);
    canvas.save("sphere.ppm").unwrap();
}

