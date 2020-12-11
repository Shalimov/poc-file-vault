use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum TransformationCrop {
  #[serde(rename = "fit")]
  FIT,
  #[serde(rename = "fill")]
  FILL,
}
