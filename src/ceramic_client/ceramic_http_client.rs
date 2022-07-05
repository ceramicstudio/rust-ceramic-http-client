use crate::ceramic_client::http_requests::CeramicHTTPRequests;
use crate::stream::genesis_commit::GenesisCommit;
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder};

pub struct CeramicHTTPClient {
    api_url: String,
    api_version: String,
    ceramic_network_url: String,
    http_client: Client,
    did: String,
}

impl CeramicHTTPClient {
    pub fn new(api_url: &str, api_version: &str, ceramic_network_url: &str) -> Self {
        Self {
            api_url: String::from(api_url),
            // TODO: properly format the api_version by prepending a "/" if it doesn't exist
            api_version: String::from(api_version),
            ceramic_network_url: String::from(ceramic_network_url),
            http_client: Client::new(),
            did: String::default(),
        }
    }

    pub fn did(&mut self, did: &str) {
        self.did = String::from(did);
    }

    pub fn get_did(&self) -> &String {
        &self.did
    }

    pub fn api_url(&mut self, api_url: &str) {
        self.api_url = String::from(api_url);
    }

    pub fn get_api_url(&self) -> &String {
        &self.api_url
    }
    pub fn api_version(&mut self, api_version: &str) {
        self.api_version = String::from(api_version);
    }
    pub fn get_api_version(&self) -> &String {
        &self.api_version
    }
    pub fn ceramic_network_url(&mut self, ceramic_network_url: &str) {
        self.ceramic_network_url = String::from(ceramic_network_url);
    }

    pub fn get_ceramic_network_url(&self) -> &String {
        &self.ceramic_network_url
    }

    pub fn http_client(&mut self, http_client: Client) {
        self.http_client = http_client;
    }
    pub fn get_http_client(&self) -> &Client {
        &self.http_client
    }
}

impl Default for CeramicHTTPClient {
    fn default() -> Self {
        Self {
            // config: Default::default(),
            api_url: String::from("https://ceramic-clay.3boxlabs.com/api"),
            api_version: String::from("/v0"),
            ceramic_network_url: String::from("https://ceramic-clay.3boxlabs.com"),
            http_client: Client::new(),
            did: String::default(),
        }
    }
}

#[async_trait]
impl CeramicHTTPRequests for CeramicHTTPClient {
    async fn get_stream(&self, stream_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String =
            format!("{}{}/streams/{}", self.api_url, self.api_version, stream_id);
        let req: RequestBuilder = self.http_client.get(endpoint);
        let res: String = req.send().await?.text().await?;
        return Result::Ok(res);
    }

    async fn get_supported_chains(&self) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String = format!("{}{}/node/chains", self.api_url, self.api_version);
        let req: RequestBuilder = self.http_client.get(endpoint);
        let res: String = req.send().await?.text().await?;
        return Result::Ok(res);
    }

    async fn get_all_stream_commits(
        &self,
        stream_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String =
            format!("{}{}/commits/{}", self.api_url, self.api_version, stream_id);
        let req: RequestBuilder = self.http_client.get(endpoint);
        let res: String = req.send().await?.text().await?;
        return Result::Ok(res);
    }

    async fn get_pin(&self, stream_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String = format!("{}{}/pins/{}", self.api_url, self.api_version, stream_id);
        let req: RequestBuilder = self.http_client.get(endpoint);
        let res: String = req.send().await?.text().await?;
        return Result::Ok(res);
    }

    async fn get_healthcheck(&self) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String = format!("{}{}/node/healthcheck", self.api_url, self.api_version);
        let req: RequestBuilder = self.http_client.get(endpoint);
        let res: String = req.send().await?.text().await?;
        return Result::Ok(res);
    }

    async fn create_stream_from_genesis(
        &self,
        genesis: GenesisCommit,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String = format!("{}{}/streams", self.api_url, self.api_version);
        let req: RequestBuilder = self.http_client.post(endpoint);
        let res: String = req.json(&genesis).send().await?.text().await?;
        return Result::Ok(res);
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn ceramic_client_gets_default_values() {
        let ceramic: CeramicHTTPClient = CeramicHTTPClient::default();
        assert_eq!(
            ceramic.get_api_url(),
            "https://ceramic-clay.3boxlabs.com/api"
        );
        assert_eq!(ceramic.get_api_version(), "/v0");
        assert_eq!(
            ceramic.get_ceramic_network_url(),
            "https://ceramic-clay.3boxlabs.com"
        );
    }

    #[test]
    fn ceramic_client_uses_new_constructor() {
        let ceramic: CeramicHTTPClient = CeramicHTTPClient::new(
            "https://ceramic-clay.3boxlabs.com/api",
            "v1",
            "https://ceramic-clay.3boxlabs.com",
        );
        assert_eq!(
            ceramic.get_api_url(),
            "https://ceramic-clay.3boxlabs.com/api"
        );
        assert_eq!(ceramic.get_api_version(), "v1");
        assert_eq!(
            ceramic.get_ceramic_network_url(),
            "https://ceramic-clay.3boxlabs.com"
        );
    }

    #[test]
    fn ceramic_client_sets_and_gets_did() {
        let mut ceramic: CeramicHTTPClient = CeramicHTTPClient::default();
        ceramic.did("did:3:0x1234567890123456789012345678901234567890");
        assert_eq!(
            ceramic.get_did(),
            "did:3:0x1234567890123456789012345678901234567890"
        );
    }

    #[test]
    fn ceramic_client_sets_and_gets_api_url() {
        let mut ceramic: CeramicHTTPClient = CeramicHTTPClient::default();
        ceramic.api_url("https://new/api");
        let api_url: &String = ceramic.get_api_url();
        assert_eq!(api_url, "https://new/api");
    }

    #[test]
    fn ceramic_client_sets_and_gets_api_version() {
        let mut ceramic: CeramicHTTPClient = CeramicHTTPClient::default();
        ceramic.api_version("v1");
        let api_version: &String = ceramic.get_api_version();
        assert_eq!(api_version, "v1");
    }

    #[test]
    fn ceramic_client_sets_and_gets_ceramic_network_url() {
        let mut ceramic: CeramicHTTPClient = CeramicHTTPClient::default();
        ceramic.ceramic_network_url("https://new-url");
        let ceramic_network_url: &String = ceramic.get_ceramic_network_url();
        assert_eq!(ceramic_network_url, "https://new-url");
    }
}
