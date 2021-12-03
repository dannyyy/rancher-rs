use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct ConfigFile {
    #[serde(rename = "CurrentServer")]
    pub current_server: String,
    #[serde(rename = "Servers")]
    pub servers: HashMap<String, Server>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub access_key: String,
    pub cacert: String,
    // pub kube_configs: Option<String>,
    // pub kube_credentials: HashMap<String, KubeCredential>,
    pub project: String,
    pub secret_key: String,
    pub token_key: String,
    pub url: String,
}
