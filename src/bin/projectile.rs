use ray_tracer::canvas::{Canvas, Color};
use ray_tracer::tuple::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = &proj.position + &proj.velocity;
    let velocity = &(&proj.velocity + &env.gravity) + &env.wind;
    Projectile { position, velocity }
}

fn main() {
    let start = Tuple::point(0.0, 1.0, 0.0);
    let velocity = Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut p = Projectile {
        position: start,
        velocity,
    };
    let gravity = Tuple::vector(0.0, -0.1, 0.0);
    let wind = Tuple::vector(-0.01, 0.0, 0.0);
    let e = Environment { gravity, wind };

    let mut c = Canvas::new(900, 550);
    let color = Color::new(1.0, 0.0, 0.0);

    while p.position.y > 0.0 {
        println!("Pos: {:?}", p.position);
        p = tick(&e, &p);
        write_square(
            &mut c,
            p.position.x as usize,
            550 - p.position.y as usize,
            &color,
        );
    }

    c.save("image.ppm").unwrap();
}

fn write_square(c: &mut Canvas, x: usize, y: usize, color: &Color) {
    if x < 5 || y < 5 {
        return;
    }
    for j in y - 5..y + 5 {
        for i in x - 5..x + 5 {
            c.write_pixel(i, j, color);
        }
    }
}
