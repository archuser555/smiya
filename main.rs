/*
Ahmed - A rust framework created to do some math for "project xy" visualation
but it can be used to animate anything! except images and polygons and text....
*/

use image::{Rgba, Rgb, RgbImage, ImageBuffer};
use nalgebra::{Matrix3, Vector3};
use std::path::Path;
use std::f32::consts::PI;

fn draw_rectangle(img: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, color: Rgb<u8>) {
    for i in x..x+width {
        for j in y..y+height {
            img.put_pixel(i, j, color);
        }
    }
}

fn draw_circle(img: &mut RgbImage, center_x: i32, center_y: i32, radius: i32, color: Rgb<u8>) {
    for x in (center_x - radius)..(center_x + radius) {
        for y in (center_y - radius)..(center_y + radius) {
            let distance = ((x - center_x) as f64).powi(2) + ((y - center_y) as f64).powi(2);
            if distance.sqrt() <= radius as f64 {
                img.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

fn draw_line(img: &mut RgbImage, x1: i32, y1: i32, x2: i32, y2: i32, color: Rgb<u8>) {
    let mut x = x1;
    let mut y = y1;
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = if dx > dy { dx } else { -dy } / 2;

    loop {
        img.put_pixel(x as u32, y as u32, color);
        if x == x2 && y == y2 {
            break;
        }
        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x += sx;
        }
        if e2 < dy {
            err += dx;
            y += sy;
        }
    }
}

fn multiply_m(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>> {
    let a_rows = a.len();
    let a_cols = a[0].len();

    let b_rows = b.len();
    let b_cols = b[0].len();

    let mut product = vec![vec![0; b_cols]; a_rows];

    if a_cols == b_rows {
        for i in 0..a_rows {
            for j in 0..b_cols {
                for k in 0..b_rows {
                    product[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        return Some(product);
    } else {
        println!("INCOMPATIBLE MATRIX SIZES");
        return None;
    }
}

fn save_image(i: i32, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let file_name = format!("img{}.png", i);
    let path = Path::new(&file_name);
    let file_path = path.to_str().unwrap();
    img.save(file_path).unwrap();
}

fn create_images(num_images: usize) -> Vec<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let mut img = Vec::with_capacity(num_images);
    for _ in 0..num_images {
        img.push(RgbImage::new(800, 800));
    }
    img
}

fn connect_points(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, i: usize, j: usize, points: &Vec<[i32; 2]>) {
    let black = Rgb([255, 0, 0]);
    let x1 = points[i][0];
    let y1 = points[i][1];
    let x2 = points[j][0];
    let y2 = points[j][1];    

    draw_line(image, x1.try_into().unwrap(), y1.try_into().unwrap(), x2.try_into().unwrap(), y2.try_into().unwrap(), black);
}

fn main() {
    let size = (400, 400);
    let mut image = create_images(500);
    let black = Rgb([0u8, 0u8, 0u8]);

    let vertices = vec![
        Vector3::new(-1.0, -1.0, -1.0),
        Vector3::new(1.0, -1.0, -1.0),
        Vector3::new(1.0, 1.0, -1.0),
        Vector3::new(-1.0, 1.0, -1.0),
        Vector3::new(-1.0, -1.0, 1.0),
        Vector3::new(1.0, -1.0, 1.0),
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(-1.0, 1.0, 1.0),
    ];

    let projection_matrix = Matrix3::new(
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    );

    let mut angles = [std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4];

    let mut projected_points = vec![];
    for i in 0..500 {
        let rotation_x = Matrix3::new(
            1.0, 0.0, 0.0,
            0.0, angles[0].cos(), -angles[0].sin(),
            0.0, angles[0].sin(), angles[0].cos(),
        );
        let rotation_y = Matrix3::new(
            angles[1].cos(), 0.0, angles[1].sin(),
            0.0, 1.0, 0.0,
            -angles[1].sin(), 0.0, angles[1].cos(),
        );
        let rotation_z = Matrix3::new(
            angles[2].cos(), -angles[2].sin(), 0.0,
            angles[2].sin(), angles[2].cos(), 0.0,
            0.0, 0.0, 1.0,
        );

        for point in &vertices {
            let point = Vector3::new(point[0], point[1], point[2]);
            let rotated2d = rotation_z * rotation_y * rotation_x * point;
            let projected2d = projection_matrix * rotated2d;
            let x = (projected2d[0] * 100.) as i32 + size.0;
            let y = (projected2d[1] * 100.) as i32 + size.1;
            projected_points.push([x, y]);
            image[i].put_pixel(x as u32, y as u32, Rgb([255, 255, 0]));
            draw_circle(&mut image[i], x, y, 10, Rgb([255, 0, 0]))
        }

        /*for p in 0..4 {
            connect_points(&mut image[i], p, (p + 1) % 4, &projected_points);
            connect_points(&mut image[i], p + 4, ((p + 1) % 4) + 4, &projected_points);
            connect_points(&mut image[i], p, p + 4, &projected_points);
        }*/
        angles[0] += std::f32::consts::FRAC_PI_4 / 10.;
        println!("{}", i);
        save_image(i as i32, &image[i as usize]);
    }
}