
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreationSuccessRessource {
    pub success: bool,
    pub uuid: String
}
