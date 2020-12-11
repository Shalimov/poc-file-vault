use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ImageExtension {
  #[serde(rename = "png")]
  PNG,
  #[serde(rename = "jpg")]
  JPG,
}
