use crate::error::{TONAPIError, TONAPIResult};
use crate::global_config::{GlobalConfig, LiteServer};
use std::iter::Cycle;
use std::vec::IntoIter;
use tokio::net::TcpStream;

/// URL address where configuration of TON MainNet is located
static MAINNET_GLOBAL_CONFIG_URL: &str = "https://ton.org/global-config.json";
/// URL address where configuration of TON TestNet is located
static TESTNET_GLOBAL_CONFIG_URL: &str = "https://ton.org/testnet-global.config.json";

/// Structure that allows rotation of liteserver addresses in case of connection issues
#[derive(Debug)]
struct LiteServerAddressRotation {
    current_address: LiteServer,
    cycle: Cycle<IntoIter<LiteServer>>,
}

impl LiteServerAddressRotation {
    /// Creates an infinite cycled iterator from list of liteserver addresses
    fn new(liteservers: Vec<LiteServer>) -> TONAPIResult<Self> {
        let mut cycle = liteservers.into_iter().cycle();
        let current_address = cycle.next().ok_or_else(|| {
            TONAPIError::LiteServerRotationError("Cannot get next liteserver address".to_string())
        })?;
        Ok(Self {
            current_address,
            cycle,
        })
    }

    /// Current liteserver address
    fn current(&self) -> &LiteServer {
        &self.current_address
    }

    /// Gets next liteserver address in iterator and replaces current() with it
    fn next(&mut self) -> TONAPIResult<&LiteServer> {
        let current = self.cycle.next().ok_or_else(|| {
            TONAPIError::LiteServerRotationError("Cannot get next liteserver address".to_string())
        })?;
        self.current_address = current;
        Ok(&self.current_address)
    }
}

/// Client for interacting with liteservers using ADNL protocol
#[derive(Debug)]
pub struct LiteServerClient {
    liteservers: LiteServerAddressRotation,
    tcp: TcpStream,
}

impl LiteServerClient {
    /// Initializing new connection inside TestNet
    pub async fn new_test_net() -> TONAPIResult<Self> {
        let config = Self::get_global_config(TESTNET_GLOBAL_CONFIG_URL).await?;
        let liteservers = LiteServerAddressRotation::new(config.liteservers)?;
        let current_literver = liteservers.current();
        let tcp = Self::init_tcp_connection(current_literver).await?;

        Ok(Self { liteservers, tcp })
    }

    /// Receiving global config via HTTP and serializing it into Rust structure
    async fn get_global_config(url: &str) -> TONAPIResult<GlobalConfig> {
        let global_config_response = reqwest::get(url).await.map_err(|err| {
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

    fn decimal_to_ip(number: i32) -> String {
        // TODO FIXME
        let mut number = number;
        let mut result = vec![];

        loop {
            if number < 256 {
                result.push(number.to_string());
                break;
            }

            let a = number / 256;
            let b = a * 256;
            number -= b;
            result.push(number.to_string());
            number = a;
        }
        result.reverse();
        result.join(".")
    }

    async fn init_tcp_connection(liteserver: &LiteServer) -> TONAPIResult<TcpStream> {
        let ip_addr = Self::decimal_to_ip(liteserver.ip);
        let address = format!("{}:{}", ip_addr, liteserver.port);
        let stream = TcpStream::connect(address)
            .await
            .map_err(|err| TONAPIError::TCPError(format!("TCP connection error: {}", err)))?;
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::LiteServerClient;

    #[test]
    fn test_decimal_to_ip() {
        assert_eq!(LiteServerClient::decimal_to_ip(1592601963), "94.237.45.107")
    }
}
