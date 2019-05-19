use std::sync::mpsc::channel;
use std::error::Error;
use std::thread;

use rayon::{ThreadPoolBuilder};
use chrono::DateTime;
use serde::{Deserialize};
use image::{DynamicImage, GenericImage, ImageBuffer, ImageFormat, RgbaImage};
use image::imageops;
use simple_error::SimpleError;


#[derive(Deserialize, Debug)]
struct LatestPayload {
    date: String,
    file: String
}

fn get_tile(time: DateTime<chrono::offset::FixedOffset>, size: u32, level: u32, x: u32, y: u32) -> Result<DynamicImage, Box<Error>> {
    let mut retries = 3;
    let base_url = format!("http://himawari8.nict.go.jp/img/D531106/{}d/{}/{}_{}_{}.png", level, size, time.format("%Y/%m/%d/%H%M%S"), x, y);

    while retries > 0 {
        if let Ok(response) = reqwest::get(&base_url) {
            let mut response = reqwest::get(&base_url)?;
            let mut buf: Vec<u8> = vec![];
            response.copy_to(&mut buf)?;
            let image = image::load_from_memory_with_format(&buf, ImageFormat::PNG)?;
            return Ok(image);
        } else {
            println!("Issue retrieving image, trying again.");
            retries = retries - 1;
        }
    }

    Err(Box::new(SimpleError::new("Failed to grab tile from the API.")))
}

struct Tile {
    image: DynamicImage,
    x: u32,
    y: u32
}

pub fn assemble(level: u32) -> Result<RgbaImage, Box<Error>> {
    let mut query_latest = reqwest::get("http://himawari8-dl.nict.go.jp/himawari8/img/D531106/latest.json")?;
    let payload: LatestPayload = query_latest.json()?;
    let time_with_tz = format!("{} +1000", payload.date);
    let time = DateTime::parse_from_str(&time_with_tz, "%Y-%m-%d %H:%M:%S %z")?;
    let tile_width = 550;
    let tile_height = 550;
    let render_width = level * tile_width;
    let render_height = level * tile_height;
    //TODO: Verify level is one of the following 4, 8, 16, 20

    let mut render: RgbaImage = ImageBuffer::new(render_width, render_height);
    let (sender, receiver) = channel();
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    // TODO: mmmmultithread
    for x in 0..level {
        for y in 0..level {
            let tx = sender.clone();
            thread::spawn(move || {
            //pool.spawn(move || {
                let image = get_tile(time, tile_width, level, x, y).unwrap();
                let data = Tile{ image: image, x: x, y: y};
                tx.send(data).unwrap();
                println!("Downloaded image for {},{}", x, y);
            });
        }
    }

    drop(sender); //Once all senders are dropped, the iter for the loop below will end.

    for tile in receiver {
        let x_pixel = tile.x * tile_width;
        let y_pixel = tile.y * tile_height;
        render.copy_from(&tile.image, x_pixel, y_pixel );
        println!("Copying for {},{}. Starting from {},{}.", tile.x, tile.y, x_pixel, y_pixel);
    }

    Ok(render)
}

pub fn render_wallpaper(path: &str, width: u32, height: u32) {
    let render = assemble(4).unwrap();
    let offset = (width - height) / 2;
    let mut wallpaper:RgbaImage = ImageBuffer::new(width, height);
    let resized = imageops::resize(&render, height, height, imageops::FilterType::Lanczos3);

    // Render a black background
    for (x, y, pixel) in wallpaper.enumerate_pixels_mut() {
        *pixel = image::Rgba([0,0,0, 255]);
    }

    let copied = wallpaper.copy_from(&resized, offset, 0);
    
    println!("Copy to wallpaper was successfull: {:?}; offset: {:?}", copied, offset);
    println!("Target path: {}", path);
    wallpaper.save(path);
}