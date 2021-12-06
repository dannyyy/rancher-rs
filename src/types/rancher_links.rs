#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RancherLinks {
    pub nodes: Option<String>,
    pub remove: Option<String>,
    #[serde(rename = "self")]
    pub _self: Option<String>,
    pub update: Option<String>,
}
