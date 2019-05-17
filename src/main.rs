use serde::{Deserialize};
use std::error::Error;
use chrono::DateTime;
use image::{DynamicImage, GenericImage, ImageBuffer, ImageFormat, RgbaImage};
use std::fs::File;
use std::io::Write;


fn main() {
    get_images().unwrap();
}

#[derive(Deserialize, Debug)]
struct LatestPayload {
    date: String,
    file: String
}

fn get_images() -> Result<(), Box<Error>> {
    let mut query_latest = reqwest::get("http://himawari8-dl.nict.go.jp/himawari8/img/D531106/latest.json")?;
    let payload: LatestPayload = query_latest.json()?;
    let time_with_tz = format!("2019-05-17 00:50:00 +1000");
    let time = DateTime::parse_from_str(&time_with_tz, "%Y-%m-%d %H:%M:%S %z")?;

    let WIDTH = 550;
    let HEIGHT = 550;
    let level = 4; // increases the quality (and the size) of each tile. possible values are 4, 8, 16, 20

    let mut tiles: Vec<DynamicImage> = vec![];

    // TODO: mmmmultithread
    for x in 0..level {
        for y in 0..level {
            //TODO: retries
            let base_url = format!("http://himawari8.nict.go.jp/img/D531106/{}d/{}/{}_{}_{}.png", level, WIDTH, time.format("%Y/%m/%d/%H%M%S"), x, y);
            let mut response = reqwest::get(&base_url)?;
            let mut buf: Vec<u8> = vec![];
            response.copy_to(&mut buf)?;
            
            let image = image::load_from_memory_with_format(&buf, ImageFormat::PNG)?;
            tiles.push(image);
            println!("Downloaded image for {},{}", x, y);
        }
    }

    tiles.reverse();

    //TODO: y'know good code practices...
    let wallpaper_width = 2200;
    let wallpaper_height = 2200;
    //FUCKING MATHS STOOPID BRAIN CMON A 9YO CAN DO THIS
    let y_offset = 0; //wallpaper_height - HEIGHT * level;
    let x_offset = 0;// wallpaper_width - WIDTH * level;

    let mut wallpaper: RgbaImage = ImageBuffer::new(wallpaper_width, wallpaper_height);

    for x in 0..level {
        for y in 0..level {
            let x_pixel = x_offset + x * WIDTH;
            let y_pixel = y_offset + y * HEIGHT;
            println!("Copying for {},{}. Starting from {},{}.", x, y, x_pixel, y_pixel);
            wallpaper.copy_from(&tiles.pop().unwrap(), x_pixel, y_pixel );
        }
    }

    wallpaper.save("yayayayayaya.png")?;

    Ok(())
}