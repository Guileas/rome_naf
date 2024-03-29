use schemars::JsonSchema;
use serde::Serialize;
use crate::models::{specialty::Specialty, naf::Naf};

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NafResource {
    pub uuid: String,
    pub code: String,
    pub label: String,
    pub description: Option<String>,
}
