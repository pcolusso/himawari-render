use std::error::Error;
use std::thread;
use std::sync::mpsc::channel;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use chrono::DateTime;
use serde::{Deserialize};
use image::{DynamicImage, GenericImage, ImageBuffer, ImageFormat, RgbaImage};
use simple_error::SimpleError;
use rayon::{ThreadPool, ThreadPoolBuilder};

#[no_mangle]
pub extern fn just_do_it() {
    let planet = assemble(4);
    planet.unwrap().save("out.png");
}

#[no_mangle]
pub extern fn save_planet(path_ref: *const c_char, level: u32) -> *const i8 {
    //let planet = assemble(level);
    let path_cstr = unsafe { CStr::from_ptr(path_ref) };

    let result = path_cstr.to_str().and_then( |path| {
        println!("Path is valid, {}", path);
        Ok(assemble(level).and_then( |rendered_image| {
            Ok(rendered_image.save(path))
        }))
    });


    let code = CString::new(match result {
        Ok(s) => "All Good",
        Err(e) => "Erk"
    }).unwrap();

    let ptr = code.as_ptr();
    std::mem::forget(code); //Leak the string

    ptr
}

#[derive(Deserialize, Debug)]
struct LatestPayload {
    date: String,
    file: String
}

//TODO: Add retries here.
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
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    // TODO: mmmmultithread
    for x in 0..level {
        for y in 0..level {
            let tx = sender.clone();
            pool.spawn(move || {
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