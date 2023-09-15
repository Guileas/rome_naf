use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewKeywordSpecialtysRequest {
    pub keywordId: String,
    pub specialtyId: String,
}
