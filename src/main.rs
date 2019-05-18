use serde::{Deserialize};
use std::error::Error;
use std::thread;
use std::sync::mpsc::channel;
use chrono::DateTime;
use image::{DynamicImage, GenericImage, ImageBuffer, ImageFormat, RgbaImage};


fn main() {
    let planet = assemble(4);
    planet.unwrap().save("out.png");
}

#[derive(Deserialize, Debug)]
struct LatestPayload {
    date: String,
    file: String
}

//TODO: Add retries here.
fn get_tile(time: DateTime<chrono::offset::FixedOffset>, size: u32, level: u32, x: u32, y: u32) -> Result<DynamicImage, Box<Error>> {
    let base_url = format!("http://himawari8.nict.go.jp/img/D531106/{}d/{}/{}_{}_{}.png", level, size, time.format("%Y/%m/%d/%H%M%S"), x, y);
    let mut response = reqwest::get(&base_url)?;
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf)?;
    let image = image::load_from_memory_with_format(&buf, ImageFormat::PNG)?;
    Ok(image)
}

struct Tile {
    image: DynamicImage,
    x: u32,
    y: u32
}

fn assemble(level: u32) -> Result<RgbaImage, Box<Error>> {
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

    // TODO: mmmmultithread
    for x in 0..level {
        for y in 0..level {
            let tx = sender.clone();
            thread::spawn(move || {
                let image = get_tile(time, tile_width, level, x, y).unwrap();
                let data = Tile{ image: image, x: x, y: y};
                tx.send(data).unwrap();
                drop(&tx);
                print!("Thread closed.");
            });
            
            println!("Downloaded image for {},{}", x, y);
        }
    }
    
    drop(sender);

    for tile in receiver {
        let x_pixel = tile.x * tile_width;
        let y_pixel = tile.y * tile_height;
        render.copy_from(&tile.image, x_pixel, y_pixel );
        println!("Copying for {},{}. Starting from {},{}.", tile.x, tile.y, x_pixel, y_pixel);
    }

    Ok(render)
}