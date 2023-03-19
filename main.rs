mod screen_resolution;

use clap::{App, Arg};
use image::{GenericImageView, ImageBuffer, Rgba};
use reqwest::blocking::get;
use std::fs::File;
use winit::dpi::PhysicalSize;

fn main() {
    let resolution = match screen_resolution::get_screen_resolution() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Warning: Failed to get screen resolution: {}. Using default 1440x900", e);
            PhysicalSize::new(1440, 900)
        }
    };

    let (width, height) = (resolution.width, resolution.height);

    let default_width_str = width.to_string();
    let default_height_str = height.to_string();

    let matches = App::new("geo_gazer")
        .version("0.0.1")
        .arg(Arg::with_name("longitude").long("lon").default_value("-74.0060"))
        .arg(Arg::with_name("latitude").long("lat").default_value("40.7128"))
        .arg(Arg::with_name("zoom").short('z').long("zoom").default_value("14"))
        .arg(
            Arg::with_name("width")
                .short('w')
                .long("width")
                .default_value(&default_width_str),
        )
        .arg(
            Arg::with_name("height")
                .short('h')
                .long("height")
                .default_value(&default_height_str),
        )
        .get_matches();

    let lon: f64 = matches.value_of("longitude").unwrap().parse().unwrap();
    let lat: f64 = matches.value_of("latitude").unwrap().parse().unwrap();
    let zoom: u32 = matches.value_of("zoom").unwrap().parse().unwrap();
    let width: u32 = matches.value_of("width").unwrap().parse().unwrap();
    let height: u32 = matches.value_of("height").unwrap().parse().unwrap();

    let (x, y) = lon_lat_to_tile_coords(lon, lat, zoom);

    // Calculate the offsets to center the provided lon/lat
    let tiles_horizontally = (width as f64 / 256.0).ceil() as i32;
    let tiles_vertically = (height as f64 / 256.0).ceil() as i32;
    let x_offset = (tiles_horizontally / 2) as i32;
    let y_offset = (tiles_vertically / 2) as i32;

    let mut stitched_image = stitch_tiles(x as i32 - x_offset, y as i32 - y_offset, zoom, width, height);
    let cropped_image = crop_to_target_resolution(&mut stitched_image, width, height);

    let mut file = File::create("output.png").unwrap();
    cropped_image
        .write_to(&mut file, image::ImageOutputFormat::Png)
        .unwrap();
}

fn lon_lat_to_tile_coords(lon: f64, lat: f64, zoom: u32) -> (u32, u32) {
    let x = ((lon + 180.0) / 360.0 * 2.0f64.powi(zoom as i32)) as u32;
    let y = ((1.0 - (lat.to_radians().tan() + (1.0 / lat.to_radians().cos())).ln() / std::f64::consts::PI) / 2.0 * 2.0f64.powi(zoom as i32)) as u32;
    (x, y)
}

fn stitch_tiles(x: i32, y: i32, zoom: u32, width: u32, height: u32) -> image::DynamicImage {

    let tile_size: u32 = 256;
    let tiles_horizontally = (width as f64 / tile_size as f64).ceil() as u32;
    let tiles_vertically = (height as f64 / tile_size as f64).ceil() as u32;

    let mut stitched_image = image::DynamicImage::new_rgba8((tiles_horizontally * tile_size) as u32, (tiles_vertically * tile_size) as u32);

    for i in 0..tiles_horizontally {
        for j in 0..tiles_vertically {
            let url = format!(
                "http://tile.stamen.com/toner/{}/{}/{}.png",
                zoom,
                x + i as i32,
                y + j as i32
            );
                let response = get(&url).unwrap();
                let img = image::load_from_memory(&response.bytes().unwrap()).unwrap();
        
                image::imageops::overlay(&mut stitched_image, &img, i * tile_size, j * tile_size);
            }
        }
        
        stitched_image
    }

    fn crop_to_target_resolution(img: &mut image::DynamicImage, width: u32, height: u32) -> image::DynamicImage {
        let (img_width, img_height) = img.dimensions();
    
        let left = (img_width - width) / 2;
        let top = (img_height - height) / 2;
    
        image::DynamicImage::ImageRgba8(image::imageops::crop(img, left, top, width, height).to_image())
    }
    


        
