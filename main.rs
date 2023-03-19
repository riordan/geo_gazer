use clap::{App, Arg};
use image::{GenericImageView, ImageBuffer, Rgba};
use reqwest::blocking::get;
use std::fs::File;

fn main() {
    let matches = App::new("Map Raster CLI")
        .version("1.0")
        .arg(
            Arg::with_name("longitude")
                .short("l")
                .long("longitude")
                .default_value("-74.0060"),
        )
        .arg(
            Arg::with_name("latitude")
                .short("a")
                .long("latitude")
                .default_value("40.7128"),
        )
        .arg(
            Arg::with_name("zoom")
                .short("z")
                .long("zoom")
                .default_value("14"),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .default_value("1440"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .default_value("900"),
        )
        .get_matches();

    let lon: f64 = matches.value_of("longitude").unwrap().parse().unwrap();
    let lat: f64 = matches.value_of("latitude").unwrap().parse().unwrap();
    let zoom: u32 = matches.value_of("zoom").unwrap().parse().unwrap();
    let width: u32 = matches.value_of("width").unwrap().parse().unwrap();
    let height: u32 = matches.value_of("height").unwrap().parse().unwrap();

    let (x, y) = lon_lat_to_tile_coords(lon, lat, zoom);
    let url = format!(
        "http://tile.stamen.com/toner/{}/{}/{}.png",
        zoom, x, y
    );

    let response = get(&url).unwrap();
    let img = image::load_from_memory(&response.bytes().unwrap()).unwrap();
    let resized = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

    let mut file = File::create("output.png").unwrap();
    resized.write_to(&mut file, image::ImageOutputFormat::Png).unwrap();
}

fn lon_lat_to_tile_coords(lon: f64, lat: f64, zoom: u32) -> (u32, u32) {
    let x = ((lon + 180.0) / 360.0 * 2.0f64.powi(zoom as i32)) as u32;
    let y = ((1.0 - (lat.to_radians().tan() + (1.0 / lat.to_radians().cos())).ln() / std::f64::consts::PI) / 2.0 * 2.0f64.powi(zoom as i32)) as u32;
    (x, y)
}
