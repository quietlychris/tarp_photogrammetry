// Image manipulation
use image::imageops::{resize, FilterType};
use image::*;
use ndarray::prelude::*;
// Machine learning
use linfa::prelude::*;
use linfa_clustering::{AppxDbscan, Dbscan};
// Path navigation and error handling
use std::error::Error;
use std::path::Path;

fn main() {
    let photo = "DJI_0137.JPG"; // 9.9 m
                                //let photo = "DJI_0139.JPG"; // 20 m
    let drone_height = 9.9; // Height of drone in meters

    // Parameter for the DBSCAN algorithm
    let scaling_factor = 0.2;
    let tolerance = 50.0;
    let min_points = 1000;
    let tarp_pixels = get_number_of_tarp_pixels(photo, scaling_factor, tolerance, min_points).unwrap();

    let est_tarp_area = area_from_pixels(drone_height, tarp_pixels, scaling_factor);

    // This tarp is nominally 8x6 ft., but I pulled out the tape measure to double-check
    // It's actually 92" x 67"
    let tarp_x = inches_to_meters(92.0);
    let tarp_y = inches_to_meters(67.0);
    let actual_tarp_area = tarp_x * tarp_y;
    println!(
        "At a height {:.2} m, est. tarp area is {:.2} m^2 vs. actual area of {:.2} m^2",
        drone_height, est_tarp_area, actual_tarp_area
    );
}

fn get_number_of_tarp_pixels(
    photo: &str,
    scaling_factor: f64,
    tolerance: f64,
    min_points: usize
) -> Result<usize, Box<dyn Error>> {
    let path = Path::new("photos/original").join(&photo);
    let img = image::open(path)?; //.to_luma();

    let (w, h) = img.dimensions();

    let img = resize(
        &img,
        (w as f64 * scaling_factor) as u32,
        (h as f64 * scaling_factor) as u32,
        FilterType::Triangle,
    );
    let (w, h) = img.dimensions();

    let mut new_img = img.clone();
    let mut array: Array2<f64> = Array2::zeros(((w * h) as usize, 5));

    // Convert this image into an Array2<f64> array with [x,y,r,g,b] rows
    for y in 0..h {
        for x in 0..w {
            let pixel = img.get_pixel(x, y);
            let num = (y * w) + x;

            array[[num as usize, 0]] = x as f64;
            array[[num as usize, 1]] = y as f64;
            array[[num as usize, 2]] = pixel[0] as f64;
            array[[num as usize, 3]] = pixel[1] as f64;
            array[[num as usize, 4]] = pixel[2] as f64;
        }
    }
    println!("Done converting image");

    let clusters = Dbscan::params(min_points)
        .tolerance(tolerance)
        .transform(&array.slice(s![.., ..]))?;

    let mut count = 0;
    for i in 0..array.shape()[0] {
        let x = array[[i, 0]] as u32;
        let y = array[[i, 1]] as u32;
        let pixel = img.get_pixel(x, y);
        match clusters[i] {
            Some(0) => {
                new_img.put_pixel(x, y, *pixel);
            }
            Some(1) => {
                new_img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
                count += 1;
            }
            None => {
                new_img.put_pixel(x, y, Rgba([0, 0, 255, 255]));
                // count +=1;
            }
            _ => {
                new_img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                println!("{},{} is something else", x, y);
            }
        }
    }

    let mod_photo = "DBSCAN_STD_".to_owned() + photo;
    let save_path = Path::new("photos/modified").join(mod_photo);
    new_img.save(save_path)?;

    Ok(count as usize)
}

fn inches_to_meters(inches: f64) -> f64 {
    inches / 12.0 / 3.28
}

fn area_from_pixels(drone_height: f64, tarp_pixels: usize, scaling_factor: f64) -> f64 {
    // DJI Mini 2 camera specs: https://www.dji.com/mini-2/specs
    // FOV: 83°
    // 35 mm format equivalent: 24 mm
    // Aperture: f/2.8
    // Focus range: 1 m to ∞
    // Image resolution: 4000x2250

    let definitely_not_a_fudge_factor = 0.5;
    let lens_angle_v: f64 = 83f64.to_radians() / 2.0;
    let lens_angle_h: f64 = (definitely_not_a_fudge_factor * 83f64).to_radians() / 2.0;

    // let d = 20.0; // distance from target [m]

    let l_v = 2.0 * drone_height * lens_angle_v.atan(); // Vertical edge length of the image frame is real distance at `d` meters
    let l_h = 2.0 * drone_height * lens_angle_h.atan();

    // println!("At {} m, the l_v is {} m, l_h is {} m", drone_height, l_v, l_h);
    let frame_area = l_v * l_h;

    let w = 4000.0 * scaling_factor; // # of pixels on the x-axis
    let h = 2250.0 * scaling_factor; // # of pixels on the y-axis
    let pixel_area = frame_area / (w * h); // Area per pixel in meters
                                           // println!("Pixel area: {:.2e} m^2", pixel_area);

    //let tarp_pixels = 102207f64; // From a quarter-size resizing of the original 4k image @ 10m
    // let tarp_pixels = 26382f64; // 20m
    let est_tarp_area = tarp_pixels as f64 * pixel_area;
    est_tarp_area
}
