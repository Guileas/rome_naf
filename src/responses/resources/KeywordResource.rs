use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeywordResource {
    pub uuid: String,
    pub label: String,
}
