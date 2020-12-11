use super::transformation_crop::TransformationCrop;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transformation {
  pub name: String,
  pub width: u16,
  pub height: u16,
  pub crop: TransformationCrop,
}
