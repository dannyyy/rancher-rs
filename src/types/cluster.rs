use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    actions: HashMap<String, String>,
    agent_features: HashMap<String, bool>,
    agent_image: String,
    agent_image_override: String,
    aks_status: HashMap<String, Option<String>>,
    allocatable: HashMap<String, String>,
    annotations: HashMap<String, String>,
    // answers:
    api_endpoint: String,
    applied_enable_network_policy: bool,
    applied_pod_security_policy_template_id: String,
    // applied_spec:
    auth_image: String,
    base_type: String,
    ca_cert: String,
    // capabilities:
    capacity: HashMap<String, String>,
    // certificates_expiration:
    cluster_template_id: String,
    cluster_template_revision_id: String,
    // component_statuses:
    created: String,
    #[serde(rename = "createdTS")]
    created_ts: i64,
    creator_id: String,
    id: String,
    labels: HashMap<String, String>,
    name: String,
}
