
use std::{io::Cursor, str::FromStr};

use chrono::{Duration, Utc, NaiveDate, NaiveDateTime, Datelike, Timelike};
use anyhow::Result;
use serde::Deserialize;
use anyhow::anyhow;
use tempdir::TempDir;
use futures::future::*;
use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};
use std::path::PathBuf;

struct Options {
    date: DateOptions,
    debug: bool,
    infrared: bool,
    zoom: i32,
    timeout: Duration,
    // temp: PathBuf
}

enum DateOptions { 
    Latest,
    Date(chrono::DateTime<Utc>)
}

impl Default for Options {
    fn default() -> Self {
        Options {
            date: DateOptions::Latest,
            debug: false,
            infrared: false,
            zoom: 2,
            timeout: Duration::seconds(30),
            // temp: TempDir::new("himawari").expect("Unable to create temp directory")
        }
    }
}

#[derive(Deserialize, Debug)]
struct Payload {
    date: String,
    file: String
}

impl Options {
    fn level(&self) -> Result<(&str, u32)> {
        match (self.infrared, self.zoom) {
            (true, 1)  => Ok(("1d", 1)),
            (true, 2)  => Ok(("4d", 4)),
            (true, 3)  => Ok(("8d", 8)),
            (false, 1) => Ok(("1d", 1)),
            (false, 2) => Ok(("4d", 4)),
            (false, 3) => Ok(("8d", 8)),
            _ => Err(anyhow!("Invalid zoom level"))
        }
    }

    async fn get_date(&self) -> Result<NaiveDateTime> {
        let image_type = if self.infrared { "INFRARED_FULL" } else { "D531106" };
        match &self.date {
            DateOptions::Latest => {
                let url = format!("https://himawari8.nict.go.jp/img/{}/latest.json", image_type);
                dbg!(&url);
                let body = reqwest::get(url)
                    .await?
                    .json::<Payload>()
                    .await?;
                dbg!(&body);
                let payload_date = NaiveDateTime::parse_from_str(&body.date, "%Y-%m-%d %H:%M:%S")?;
                let date = NaiveDate::from_ymd(payload_date.year(), payload_date.month(), payload_date.day())
                    .and_hms(payload_date.hour(), payload_date.minute() - payload_date.minute() % 10, 0);
                dbg!(&date);
                Ok(date)
            }
            DateOptions::Date(d) => unimplemented!()
        }
    }

    async fn assemble(&self) -> Result<Vec<DynamicImage>> {
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
            &date.format("%H%M%S").to_string()
        ].join("/");
        let mut images = vec!();
        

        // Dunno if the below could be optimised better, probably gets blocked waiting for images.

        for x in 0..blocks {
            for y in 0..blocks {
                let url = format!("{}_{}_{}.png", base_url, x, y);
                dbg!(&url);
                let req = client.get(url).send().await?;
                if req.status().is_success() {
                    let bytes = req.bytes().await?;
                    let mut reader = ImageReader::new(Cursor::new(bytes));
                    reader.set_format(ImageFormat::Png);
                    images.push(reader.decode()?);
                    dbg!(images.len());
                } else {
                    Err(anyhow!("Err"))?;
                }
                
            }
        }

        Ok(images)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::runtime::Runtime;

    use crate::*;

    #[test]
    fn run() -> Result<()> {
        let runtime = Runtime::new()?;
        let options = Options::default();
        runtime.block_on(options.assemble())?;

        Ok(())
    }
}