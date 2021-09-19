use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewKeywordNafsRequest {
    pub keywordId: String,
    pub nafId: String,
}
