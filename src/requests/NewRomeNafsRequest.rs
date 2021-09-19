use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewRomeNafsRequest {
    pub romeId: String,
    pub nafId: String,
}
