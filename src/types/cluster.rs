use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub actions: HashMap<String, String>,
    pub agent_features: Option<HashMap<String, bool>>,
    pub agent_image: String,
    pub agent_image_override: String,
    pub aks_status: HashMap<String, Option<String>>,
    pub allocatable: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
    // answers:
    pub api_endpoint: String,
    pub applied_enable_network_policy: bool,
    pub applied_pod_security_policy_template_id: String,
    // applied_spec:
    pub auth_image: String,
    pub base_type: String,
    pub ca_cert: String,
    // capabilities:
    pub capacity: HashMap<String, String>,
    // certificates_expiration:
    pub cluster_template_id: String,
    pub cluster_template_revision_id: String,
    // component_statuses:
    pub created: String,
    #[serde(rename = "createdTS")]
    pub created_ts: i64,
    pub creator_id: String,
    pub id: String,
    pub labels: HashMap<String, String>,
    pub name: String,
}
