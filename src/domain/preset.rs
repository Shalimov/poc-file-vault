use super::image_extension::ImageExtension;
use super::transformation::Transformation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Preset {
  pub dprs: Vec<u8>,
  pub extensions: Vec<ImageExtension>,
  pub transformations: Vec<Transformation>,
}
