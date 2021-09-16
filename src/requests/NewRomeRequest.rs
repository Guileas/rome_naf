use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewRomeRequest {
    pub code: String,
    pub label: String,
    pub description: Option<String>,
}
