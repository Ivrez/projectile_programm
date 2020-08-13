use std::io::BufRead;
use std::f64::consts;

const gravity: f64 = 9.8;

fn main() {
    println!("projectile");

    //let x: i32 = 0;
    //let y: i32 = 0;

    //println!("input your position x:");
    //x = input_parameter();

    //println!("input your position y:");
    //y = input_parameter();

    //println!("x, y: {} {}", x, y);

    let launch_speed = 100.0;
    let launch_angle = 75.0;

    let launch_angle_rad = deg_to_rad(launch_angle);
    let sin_angle = launch_angle_rad.sin();
    let cos_angle = launch_angle_rad.cos();

    let t = time_of_flight(launch_speed, sin_angle);
    let h = max_height(launch_speed, sin_angle);
    let d = range(launch_speed, sin_angle, cos_angle);

    //let v0x; 
    //let v0y; 

    //v0x = v0 * ang_rad.cos();
    //v0y = v0 * ang_rad.sin();


    //println!("{} {}", v0x, v0y);
    println!("{}", t);
    println!("{}", h);
    println!("{}", d);

}

fn deg_to_rad(deg: f64) -> f64{
    let rad = deg * consts::PI/180.0;

    rad
}

fn time_of_flight(launch_speed: f64, sin_angle: f64) -> f64{  // projectile time flight
    let time = (launch_speed * sin_angle + ((launch_speed * sin_angle).powi(2) + 2.0 * gravity).sqrt()) / gravity;
    //let t = (2.0 * v0 / g) * ang_rad.sin();

    time
}

fn max_height(launch_speed: f64, sin_angle: f64) -> f64{
    let height = launch_speed.powi(2) * sin_angle.powi(2) / (2.0 * gravity);

    height
}

fn range(launch_speed: f64, sin_angle: f64, cos_angle: f64) -> f64{
    let distance = (launch_speed.powi(2) / gravity) * sin_angle * cos_angle * 2.0;

    distance
}


fn input_parameter() -> i32{
    let n: i32= std::io::stdin()
        .lock()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("couldn't read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer");

    n
}
