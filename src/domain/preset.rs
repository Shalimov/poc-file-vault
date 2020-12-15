use super::image_extension::ImageExtension;
use super::image_info::ImageInfo;
use super::transformation::Transformation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Preset {
  pub dprs: Vec<u8>,
  pub extensions: Vec<ImageExtension>,
  pub transformations: Vec<Transformation>,
}

impl Preset {
  fn to_image_info(
    &self,
    image_uid: String,
    size: (u16, u16),
    origina_extension: ImageExtension,
  ) -> Vec<ImageInfo> {
    let mut image_infos: Vec<ImageInfo> = vec![];

    for dpr in self.dprs {
      for ext in self.extensions {
        for transformation in self.transformations {

          let file_name = match dpr {
            1 => format!("{}.{}", transformation.name, ext.to_string()),
            value => format!("{}_@{}x.{}", transformation.name, value, ext.to_string())
          };

          let image_info = ImageInfo {
            file_name: file_name,
            original: false,
            extension: ext,
            height: 0,
            width: 0
          };

          image_infos.push(image_info);
        }
      }
    }

    // self.dprs.into_iter().fold(&mut image_infos, |infos: &mut Vec<ImageInfo>, dpr: u8| {
    //   infos.push(value: T)
    // });

    image_infos
  }
}
