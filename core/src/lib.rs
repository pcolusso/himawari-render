use anyhow::{anyhow, Error, Result};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike, Utc};
use image::{
    imageops, io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView, ImageFormat,
    RgbaImage,
};
use serde::Deserialize;
use std::ffi::CStr;
use std::ffi::CString;
use std::io::Cursor;
use std::os::raw::c_char;

const SIZE: u32 = 550;

pub struct Options {
    date: DateOptions,
    infrared: bool,
    zoom: u32,
    timeout: Duration,
}

pub enum DateOptions {
    Latest,
    Date(chrono::DateTime<Utc>),
}

impl Default for Options {
    fn default() -> Self {
        Options {
            date: DateOptions::Latest,
            infrared: false,
            zoom: 2,
            timeout: Duration::seconds(30),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Payload {
    date: String,
    file: String,
}

struct Tile {
    image: DynamicImage,
    x: u32,
    y: u32,
}

impl Options {
    fn level(&self) -> Result<(&str, u32)> {
        match (self.infrared, self.zoom) {
            (true, 1) => Ok(("1d", 1)),
            (true, 2) => Ok(("4d", 4)),
            (true, 3) => Ok(("8d", 8)),
            (false, 1) => Ok(("1d", 1)),
            (false, 2) => Ok(("4d", 4)),
            (false, 3) => Ok(("8d", 8)),
            _ => Err(anyhow!("Invalid zoom level")),
        }
    }
    async fn get_date(&self) -> Result<NaiveDateTime> {
        let image_type = if self.infrared {
            "INFRARED_FULL"
        } else {
            "D531106"
        };
        match &self.date {
            DateOptions::Latest => {
                let url = format!(
                    "https://himawari8.nict.go.jp/img/{}/latest.json",
                    image_type
                );
                dbg!(&url);
                let body = reqwest::get(url).await?.json::<Payload>().await?;
                dbg!(&body);
                let payload_date = NaiveDateTime::parse_from_str(&body.date, "%Y-%m-%d %H:%M:%S")?;
                let date = NaiveDate::from_ymd(
                    payload_date.year(),
                    payload_date.month(),
                    payload_date.day(),
                )
                .and_hms(
                    payload_date.hour(),
                    payload_date.minute() - payload_date.minute() % 10,
                    0,
                );
                dbg!(&date);
                Ok(date)
            }
            DateOptions::Date(d) => unimplemented!(),
        }
    }

    async fn get_images(&self) -> Result<Vec<Tile>> {
        let date = self.get_date().await?;
        let (level, blocks) = self.level()?;
        let client = reqwest::Client::new();
        let base_url = [
            "https://himawari8.nict.go.jp/img",
            "D531106", // TODO: substitute if infrared
            level,
            "550",
            &date.format("%Y").to_string(),
            &date.format("%m").to_string(),
            &date.format("%d").to_string(),
            &date.format("%H%M%S").to_string(),
        ]
        .join("/");
        let mut urls = vec![];
        let mut images = vec![];

        for x in 0..blocks {
            for y in 0..blocks {
                urls.push((format!("{}_{}_{}.png", base_url, x, y), x, y));
            }
        }

        #[cfg(feature = "wasm")]
        {
            let mut bodies = vec![];
            for url in urls {
                let resp = client.get(url.0).send().await?;
                let bytes = resp.bytes().await?;
                let mut reader = ImageReader::new(Cursor::new(bytes));
                reader.set_format(ImageFormat::Png);
                let image = reader.decode()?;
                let x = url.1;
                let y = url.2;
                bodies.push(Ok::<Tile, Error>(Tile { image, x, y }))
            }

            for i in bodies {
                match i {
                    Ok(image) => images.push(image),
                    Err(e) => Err(anyhow!("Issue processing images, {}", e))?,
                }
            }
        }
        #[cfg(not(feature = "wasm"))]
        {
            let bodies = futures::future::join_all(urls.into_iter().map(|url| {
                let client = client.clone();
                tokio::spawn(async move {
                    let resp = client.get(url.0).send().await?;
                    let bytes = resp.bytes().await?;
                    let mut reader = ImageReader::new(Cursor::new(bytes));
                    reader.set_format(ImageFormat::Png);
                    let image = reader.decode()?;
                    let x = url.1;
                    let y = url.2;
                    Ok::<Tile, Error>(Tile { image, x, y })
                })
            }))
            .await;

            for i in bodies {
                match i {
                    Ok(Ok(image)) => images.push(image),
                    Ok(Err(e)) => Err(anyhow!("Issue processing images, {}", e))?,
                    Err(e) => Err(anyhow!("Error scheduling image download, {}", e))?,
                }
            }
        }

        Ok(images)
    }

    pub async fn build_image(&self) -> Result<DynamicImage> {
        let tiles = self.get_images().await?;
        let count = self.level()?.1;
        let mut image = RgbaImage::new(SIZE * count, SIZE * count);

        for tile in tiles {
            image.copy_from(&tile.image, tile.x * SIZE, tile.y * SIZE)?;
        }

        imageops::colorops::contrast_in_place(&mut image, 0.5);

        Ok(image::DynamicImage::ImageRgba8(image))
    }

    pub async fn build_wallpaper(
        &self,
        width: u32,
        height: u32,
        image: &DynamicImage,
    ) -> Result<DynamicImage> {
        let scale_factor = height as f32 / image.height() as f32;
        let new_size = image.height() as f32 * scale_factor;
        dbg!(scale_factor, new_size);

        let mut new_image = image::imageops::resize(
            image,
            new_size as u32,
            new_size as u32,
            imageops::FilterType::Lanczos3,
        );
        let x = (width - new_image.width()) / 2;
        let y = (height - new_image.height()) / 2;

        dbg!(new_image.width(), new_image.height(), x, y);

        let mut wallpaper = RgbaImage::new(width, height);
        wallpaper.copy_from(&mut new_image, x, y)?;

        Ok(image::DynamicImage::ImageRgba8(wallpaper))
    }
}

#[cfg(not(feature = "wasm"))]
fn single_wallpaper(width: u32, height: u32, quality: u32) -> Result<DynamicImage> {
    let runtime = tokio::runtime::Runtime::new()?;
    let options = Options {
        zoom: quality,
        ..Default::default()
    };
    runtime.block_on(async {
        let image = options.build_image().await?;
        let wallpaper = options.build_wallpaper(width, height, &image).await?;
        Ok::<DynamicImage, anyhow::Error>(wallpaper)
    })
}

#[cfg(not(feature = "wasm"))]
fn extern_save_file(file: *const c_char, image: DynamicImage) -> Result<()> {
    let path = unsafe { CStr::from_ptr(file) };
    dbg!(&path);
    image.save_with_format(path.to_str()?, ImageFormat::Jpeg)?;

    Ok(())
}

#[cfg(not(feature = "wasm"))]
#[no_mangle]
pub extern "C" fn single_wallpaper_file(
    width: u32,
    height: u32,
    quality: u32,
    file: *const c_char,
) -> *mut c_char {
    let res = single_wallpaper(width, height, quality);

    match res {
        Ok(image) => match extern_save_file(file, image) {
            Ok(()) => CString::new("Success"),
            Err(e) => CString::new(format!("Failed, {}", e)),
        },
        Err(e) => CString::new(format!("Failed, {}", e)),
    }
    .unwrap()
    .into_raw()
}

#[cfg(feature = "wasm")]
mod wasm {
    use crate::Options;
    use anyhow::Error;
    use image::GenericImageView;
    use wasm_bindgen::{prelude::*, Clamped, JsCast};
    use wasm_bindgen_futures::JsFuture;
    use web_sys::ImageData;

    #[wasm_bindgen]
    pub async fn create_wallpaper(
        width: u32,
        height: u32,
        quality: u32,
        canvas: String,
    ) -> Result<(), JsValue> {
        web_sys::console::log_1(
            &format!("Starting... Creating an image of {}x{}", width, height).into(),
        );
        // Generate image...
        let options = Options {
            zoom: quality,
            ..Default::default()
        };
        let image = options
            .build_image()
            .await
            .expect("Could not generate image");
        web_sys::console::log_1(&"Image created".into());

        let wallpaper = options
            .build_wallpaper(width, height, &image)
            .await
            .expect("Could not generate wallpaper");
        web_sys::console::log_1(
            &format!(
                "Wallpaper created; dimensions: {}x{}",
                wallpaper.width(),
                wallpaper.height(),
            )
            .into(),
        );

        let mut rgba_image = wallpaper.to_rgba8();
        
        // Set the canvas
        let window = web_sys::window().expect("Could not get window");
        web_sys::console::log_1(&"Window found".into());
        let document = window.document().expect("Could not get document");
        web_sys::console::log_1(&"Document found".into());
        let canvas = document
            .get_element_by_id(&canvas)
            .expect("Could not get canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        web_sys::console::log_1(&"Canvas found".into());
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        web_sys::console::log_1(&"Context found".into());
        
        let clamped_buf: Clamped<&[u8]> = Clamped(rgba_image.as_raw());
        web_sys::console::log_1(
            &format!(
                "Copying to canvas with expected dimensions {}x{}",
                wallpaper.width(),
                wallpaper.height()
            )
            .into(),
        );
        let image_data_temp = ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, width, height)?;
        context.put_image_data(&image_data_temp, 0.0, 0.0)?;

        Ok(())
    }
}
