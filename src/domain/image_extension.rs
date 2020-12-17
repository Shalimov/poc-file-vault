use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ImageExtension {
  #[serde(rename = "png")]
  PNG,
  #[serde(rename = "jpg")]
  JPG,
}

impl ToString for ImageExtension {
  fn to_string(&self) -> String {
    match self {
      ImageExtension::JPG => "jpg".to_owned(),
      ImageExtension::PNG => "png".to_owned(),
    }
  }
}
