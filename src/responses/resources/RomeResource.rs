use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RomeResource {
    pub uuid: String,
    pub code: String,
    pub label: String,
    pub description: Option<String>,
}
