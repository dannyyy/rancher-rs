use crate::types::{Collection, NodePool};
use eyre::eyre;

mod types;

#[derive(Debug)]
pub struct RancherClient {
    bearer_token: String,
    http: reqwest::Client,
    base_url: reqwest::Url,
}

impl RancherClient {
    pub fn new(bearer_token: String, base_url: String) -> Self {
        Self {
            bearer_token,
            http: reqwest::Client::builder().build().unwrap(),
            base_url: base_url.parse().unwrap(),
        }
    }

    pub fn from_config_file(rancher_host: &str) -> eyre::Result<Self> {
        let home_dir = dirs_next::home_dir();
        let home_dir = if let Some(p) = home_dir {
            p
        } else {
            return Err(eyre!("missing home dir"));
        };

        let cli2_path = home_dir.join(".rancher/cli2.json");
        if !cli2_path.exists() {
            return Err(eyre!("rancher's ~/.rancher/cli2.json is missing"));
        }

        let cli2_data = std::fs::read_to_string(cli2_path)?;
        let parsed_cli2: crate::types::ConfigFile = serde_json::from_slice(cli2_data.as_bytes())?;
        // let parsed_cli2: serde_json::Value = serde_json::from_slice(cli2_data.as_bytes())?;
        // parsed_cli2["Servers"].as_object().unwrap().iter().map(|(k,v)| (k,v)| )

        let server_names: Vec<_> = parsed_cli2
            .servers
            .iter()
            .map(|(k, _)| k.to_owned())
            .collect();
        let server = parsed_cli2
            .servers
            .into_iter()
            .find(|(k, _)| k.contains(rancher_host));

        if server.is_none() {
            return Err(eyre!(
                "could not find server {} in {:?}",
                rancher_host,
                server_names
            ));
        }
        let (_, server_spec) = server.unwrap();

        Ok(RancherClient::new(server_spec.token_key, server_spec.url))
    }

    pub async fn node_pools(&self) -> eyre::Result<Collection<NodePool>> {
        let nodepools = self
            .http
            .get(self.base_url.join("v3/nodepools").unwrap())
            .bearer_auth(&self.bearer_token)
            .send()
            .await?;

        let bytes = nodepools.bytes().await?;
        let string = String::from_utf8(bytes.to_vec())?;

        let collection: Collection<NodePool> = serde_json::from_str(&string)?;

        Ok(collection)
    }

    pub async fn set_drain_before_delete(
        &self,
        cluster_id: &str,
        node_pool_id: &str,
    ) -> eyre::Result<()> {
        let url = self
            .base_url
            .join(&format!("v3/nodePools/{}:{}", cluster_id, node_pool_id))
            .unwrap();
        let res = self
            .http
            .get(url.clone())
            .bearer_auth(&self.bearer_token)
            .send()
            .await?;

        let bytes = res.bytes().await.unwrap();
        let mut value: serde_json::Value = serde_json::from_slice(&bytes[..])?;

        value["drainBeforeDelete"] = serde_json::Value::Bool(true);

        let res = self
            .http
            .put(url)
            .bearer_auth(&self.bearer_token)
            .json(&value)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(eyre!(
                "rancher api returned error: {:?} {}",
                res.status(),
                String::from_utf8(res.bytes().await?.to_vec())?
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_config_file() {
        RancherClient::from_config_file("https://console.aws.dockyard.viasat.io").unwrap();
    }

    #[tokio::test]
    async fn test_get_node_pools() {
        let rc = RancherClient::from_config_file("https://console.aws.dockyard.viasat.io").unwrap();

        rc.node_pools().await.unwrap();
    }

    #[tokio::test]
    async fn test_update_node_pool() {
        let rc = RancherClient::new(
            "token-sp8dq:758k5rm9rtd6lbvprvtk2nmfr2wgctj8xc752mm9hdgfhwhnbnk77v".into(),
            "https://console.aws.rancher.viasat.io".into(),
        );

        for np in &["np-6qlqc", "np-t9n54", "np-xbtg6"] {
            rc.set_drain_before_delete("c-zcwzm", *np).await.unwrap();
        }
    }
}
