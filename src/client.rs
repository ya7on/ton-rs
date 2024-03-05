use crate::error::{TONAPIError, TONAPIResult};
use crate::global_config::GlobalConfig;

static TESTNET_GLOBAL_CONFIG: &str = "https://ton.org/testnet-global.config.json";

#[derive(Debug)]
pub struct LiteServerClient {
    config: GlobalConfig,
}

impl LiteServerClient {
    pub async fn new_test_net() -> TONAPIResult<Self> {
        let config = Self::get_global_config(TESTNET_GLOBAL_CONFIG).await?;

        Ok(Self { config })
    }

    async fn get_global_config(url: &str) -> TONAPIResult<GlobalConfig> {
        let global_config_response = reqwest::get(TESTNET_GLOBAL_CONFIG).await.map_err(|err| {
            TONAPIError::GlobalConfigError(format!("Cannot send request to global config: {}", err))
        })?;
        if !global_config_response.status().is_success() {
            return Err(TONAPIError::GlobalConfigError(
                "Non success response from global config url".to_string(),
            ));
        }
        let bytes = global_config_response.bytes().await.map_err(|err| {
            TONAPIError::GlobalConfigError(format!(
                "Cannot read response from global config: {}",
                err
            ))
        })?;
        serde_json::from_slice::<GlobalConfig>(&bytes).map_err(|err| {
            TONAPIError::GlobalConfigError(format!("Global config parsing error: {}", err))
        })
    }

    fn establish_connection(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::client::LiteServerClient;

    #[tokio::test]
    async fn test_() {
        let client = LiteServerClient::new_test_net().await.unwrap();

        panic!("{:?}", client);
    }
}
