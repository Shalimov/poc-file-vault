pub enum ImageExtension {
  PNG,
  JPG
}

impl ImageExtension {
  pub fn parse(value: &str) -> Result<ImageExtension, &str> {
    match value {
      "png" => Ok(ImageExtension::PNG),
      "jpg" => Ok(ImageExtension::JPG),
      value => Err(format!("Wrong extension is passed: {}", value))
    }
  }
}

pub enum TransformationCrop {
  FIT,
  FILL
}

impl TransformationCrop {
  pub fn parse(value: &str) -> Result<TransformationCrop, &str> {
    match value {
      "fit" => Ok(TransformationCrop::FIT),
      "fill" => Ok(TransformationCrop::FILL),
      value => Err(format!("Wrong transformation crop is passed: {}", value))
    }
  }
}

pub struct Transformation {
  name: String,
  width: u16,
  height: u16,
  crop: TransformationCrop
}

pub struct Preset {
  pub dprs: Vec<f32>,
  pub extensions: Vec<ImageExtension>,
  pub transformations: Vec<Transformation>
}
