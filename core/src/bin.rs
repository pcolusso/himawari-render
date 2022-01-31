use std::{path::PathBuf, process::Command};

use himawari_render::Options;
use anyhow::Result;
use clap::Parser;
use image::ImageFormat;

#[derive(Parser)]
#[clap(name = "himawari_cli")]
#[clap(bin_name = "himawari_cli")]
enum Commands {
  Wallpaper(WallpaperCommand),
  Image(ImageCommand)
}

#[derive(clap::Args)]
struct WallpaperCommand {
  #[clap(short, long, default_value_t = 1920)]
  width: u32,
  #[clap(short, long, default_value_t = 1080)]
  height: u32,
  location: PathBuf
}

#[derive(clap::Args)]
struct ImageCommand {
  #[clap(short, long, default_value_t = 2)]
  quality: u32,
  #[clap(short, long, default_value = "image.jpg")]
  location: PathBuf
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Commands::parse();

  match args {
    Commands::Image(opts) => {
      let options = Options { zoom: opts.quality, ..Options::default() };
      let image = options.build_image().await?;
      image.save_with_format(opts.location, ImageFormat::Jpeg)?;
    },
    Commands::Wallpaper(_) => {
      let options = Options::default();
      let image = options.build_image().await?;
      options.build_wallpaper(3840, 2160, &image).await?;
    }
  }
  
  Ok(())
}