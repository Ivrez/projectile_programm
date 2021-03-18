use std::io::BufRead;
use std::f32::consts;

const GRAVITY: f32 = 9.8;

mod draw;

fn main() {
    println!("projectile");

    println!("input lanuch speed:");
    let launch_speed: f32 = input_parameter();

    println!("input launch angle:");
    let launch_angle: f32 = input_parameter();

    let launch_angle_rad = deg_to_rad(launch_angle);
    let sin_angle = launch_angle_rad.sin();
    let cos_angle = launch_angle_rad.cos();

    let t = time_of_flight(launch_speed, sin_angle);
    let h_max = max_height(launch_speed, sin_angle);
    let d = range(launch_speed, sin_angle, cos_angle);

    println!("launch_speed: {}", launch_speed);
    println!("launch_angle: {}", launch_angle);

    println!("sin_angle: {}", sin_angle);
    println!("cos_angle: {}", cos_angle);

    println!("t: {}", t);
    println!("h_max: {}", h_max);
    println!("d: {}", d);

    let mut coordinates = Vec::new();
    coordinates.push((0f32, 0f32));

    let mut x;
    let mut y;
    let mut n;

    for i in 1..=20 {

        n = t / 200.0 * (i as f32 * 10.0);
        x = n * launch_speed * cos_angle;
        y = n * launch_speed * sin_angle - ((0.5 * GRAVITY) * n.powi(2));

        coordinates.push((x, y));
    }

    coordinates.clone().into_iter().for_each(|it| {
            println!("{:#?}", it);
    });

    draw::draw_graphics(coordinates, d, h_max).map_err(|err| println!("{:?}", err)).ok();
}

fn input_parameter() -> f32{
  let n: f32 = std::io::stdin()
    .lock()
    .lines()
    .next()
    .expect("stdin should be available")
    .expect("couldn't read from stdin")
    .trim()
    .parse()
    .expect("input was not an float");

  n
}

fn deg_to_rad(deg: f32) -> f32{
    let rad = deg * consts::PI/180.0;

    rad
}

fn time_of_flight(launch_speed: f32, sin_angle: f32) -> f32{  // projectile time flight
    let time = (launch_speed * sin_angle + ((launch_speed * sin_angle).powi(2) + 2.0 * GRAVITY).sqrt()) / GRAVITY;

    time
}

fn max_height(launch_speed: f32, sin_angle: f32) -> f32{
    let height = launch_speed.powi(2) * sin_angle.powi(2) / (2.0 * GRAVITY);

    height
}

fn range(launch_speed: f32, sin_angle: f32, cos_angle: f32) -> f32{
    let distance = (launch_speed.powi(2) / GRAVITY) * sin_angle * cos_angle * 2.0;

    distance
}
