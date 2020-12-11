use crate::domain::preset::Preset;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadInput {
  pub organization_uid: String,
  pub space_name: String,
  pub preset_name: Option<String>,
  pub preset: Option<Preset>,
  pub tags: Option<HashMap<String, String>>,
}
