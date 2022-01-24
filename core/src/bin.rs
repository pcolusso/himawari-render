use himawari_render::Options;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let options = Options::default();
  let image = options.build_image().await?;
  options.build_wallpaper(3840, 2160, &image).await?;

  Ok(())
}