use super::image_extension::ImageExtension;

pub struct ImageInfo {
  pub file_name: String,
  pub extension: ImageExtension,
  pub original: bool,
  pub width: u16,
  pub height: u16,
}
