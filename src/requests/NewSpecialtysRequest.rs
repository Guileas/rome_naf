use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewSpecialtyRequest {
    pub label: String,
    pub description: Option<String>,
    pub naf_id: String,
}
