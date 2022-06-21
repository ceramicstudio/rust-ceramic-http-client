use reqwest::Client;

#[derive(Debug)]
pub struct CeramicConfig {
    pub client: Client,
    pub api_version: String,
    pub ceramic_url: String,
}

impl CeramicConfig {
    pub fn new(
        network: &str,
        api_version: &str,
        client: Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client,
            api_version: format!("/{}", api_version),
            ceramic_url: match network {
                "local" => String::from("http://127.0.0.1:7007/api"),
                "clay" => String::from("https://ceramic-clay.3boxlabs.com/api"),
                &_ => {
                    return Result::Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "unknown network",
                    )))
                }
            },
        })
    }
}

impl Default for CeramicConfig {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_version: String::from("/v0"),
            ceramic_url: String::from("https://ceramic-clay.3boxlabs.com/api"),
        }
    }
}
