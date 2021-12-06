use crate::types::RancherLinks;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection<T> {
    #[serde(rename = "type")]
    pub _type: String,
    pub links: RancherLinks,
    pub create_types: HashMap<String, String>,
    pub actions: Option<HashMap<String, String>>,
    pub pagination: HashMap<String, i32>,
    // sort:
    pub filters: HashMap<String, Option<String>>,
    pub resource_type: String,
    pub data: Vec<T>,
}
