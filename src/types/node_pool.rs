use super::RancherLinks;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePool {
    pub base_type: String,
    pub cluster_id: String,
    pub control_plane: bool,
    pub created: String,
    #[serde(rename = "createdTS")]
    pub created_ts: i64,
    pub creator_id: String,
    pub delete_not_ready_after_secs: i64,
    pub display_name: Option<String>,
    pub drain_before_delete: bool,
    pub driver: String,
    pub etcd: bool,
    pub hostname_prefix: String,
    pub id: String,
    pub labels: HashMap<String, String>,
    pub links: RancherLinks,
}
