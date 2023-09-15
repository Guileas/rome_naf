use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpecialtyResource {
    pub uuid: String,
    pub label: String,
    pub description: Option<String>,
    pub keyword_id: String,
}

