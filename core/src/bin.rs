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
  #[clap(default_value_t = 1170 )]
  width: u32,
  #[clap(default_value_t = 2532 )]
  height: u32,
  #[clap(short, long, default_value_t = 2)]
  quality: u32,
  #[clap(short, long, default_value = "image.jpg")]
  output: PathBuf
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
    Commands::Wallpaper(opts) => {
      let options = Options { zoom: opts.quality, ..Options::default() };
      let image = options.build_image().await?;
      let wallpaper = options.build_wallpaper(opts.width, opts.height, &image).await?;
      wallpaper.save_with_format(opts.output, ImageFormat::Jpeg)?;
    }
  }
  
  Ok(())
}