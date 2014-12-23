extern crate image;
extern crate noise;

use std::rand;
use std::num::Float;

static HEIGHT: u32 = 800;
static WIDTH: u32 = 1200;
static ZOOM: f32 = 100.0;

fn main() {
    let seed = &noise::Seed::new(rand::random());
    // let seed = &noise::Seed::new(42424242424242);
    let mut pixels = Vec::with_capacity((WIDTH * HEIGHT * 3) as uint);

    for y in range(0, HEIGHT) {
        for x in range(0, WIDTH) {
            let point = &pixel_to_point(x, y);
            let value = value_at_point(seed, point);
            let color = &color_for_value(value);

            pixels.push_all(color);
        }

        println!("{}", y);
    }

    let filename = "foobar.png";
    let _ = image::save_buffer(&Path::new(filename), pixels.as_slice(),
                               WIDTH, HEIGHT, image::ColorType::RGB(8));
}

fn color_for_value(value: f32) -> [u8, ..3] {
    let colors: &[(f32, [u8, ..3])] = &[
        (-1.000, [  0,   0, 128]),
        (-0.250, [  0,   0, 255]),
        ( 0.000, [  0, 128, 255]),
        ( 0.062, [240, 240,  64]),
        ( 0.125, [ 32, 160,   0]),
        ( 0.375, [224, 224,   0]),
        ( 0.550, [128, 128, 128]),
        ( 1.000, [255, 255, 255])
    ];

    for i in range(0, colors.len()) {
        let (height, color) = colors[i];

        if value < height {
            let (prev_height, prev_color) = colors[i - 1];

            let n = (value - prev_height) / (height - prev_height);
            return lerp_colors(prev_color, color, n);
        }
    }

    panic!("This should not happen!");
}

fn lerp_colors(a: [u8, ..3], b: [u8, ..3], n: f32) -> [u8, ..3] {
    [
        lerp(a[0] as f32, b[0] as f32, n),
        lerp(a[1] as f32, b[1] as f32, n),
        lerp(a[2] as f32, b[2] as f32, n)
    ]
}

fn lerp(a: f32, b: f32, n: f32) -> u8 {
    (a + (b - a) * n) as u8
}

fn value_at_point(seed: &noise::Seed, point: &noise::Point2<f32>) -> f32 {
    let adjusted = &adjust_point(point);
    let height = noise::brownian2(seed, adjusted, noise::perlin2_best, 1.7, 8);

    let gradient = gradient_value(point);

    clamp(height + gradient, -1.0, 1.0)
}

fn adjust_point(point: &noise::Point2<f32>) -> noise::Point2<f32> {
    [point[0] / ZOOM, point[1] / ZOOM]
}

fn gradient_value(point: &noise::Point2<f32>) -> f32 {
    let dist_x = point[0] - (WIDTH as f32) / 2.0;
    let dist_y = point[1] - (HEIGHT as f32) / 2.0;

    let max_dist_x = WIDTH as f32 / 2.0;
    let max_dist_y = HEIGHT as f32 / 2.0;

    let dx = (dist_x / max_dist_x).abs();
    let dy = (dist_y / max_dist_y).abs();

    0.2 - (dx * dx * dx + dy * dy * dy).powf(1.0 / 3.0)
}

fn clamp(n: f32, min_bound: f32, max_bound: f32) -> f32 {
    if n > max_bound {
        max_bound
    } else if n < min_bound {
        min_bound
    } else {
        n
    }
}

fn pixel_to_point(x: u32, y: u32) -> noise::Point2<f32> {
    [(x as f32), (y as f32)]
}

