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

    pub fn set_did(&mut self, did: &str) {
        self.did = String::from(did);
    }

    pub fn get_did(&self) -> &String {
        &self.did
    }

    pub fn set_api_url(&mut self, api_url: &str) {
        self.api_url = String::from(api_url);
    }

    pub fn get_api_url(&self) -> &String {
        &self.api_url
    }
    pub fn set_api_version(&mut self, api_version: &str) {
        self.api_version = String::from(api_version);
    }
    pub fn get_api_version(&self) -> &String {
        &self.api_version
    }
    pub fn set_ceramic_network_url(&mut self, ceramic_network_url: &str) {
        self.ceramic_network_url = String::from(ceramic_network_url);
    }

    pub fn get_ceramic_network_url(&self) -> &String {
        &self.ceramic_network_url
    }

    pub fn set_http_client(&mut self, http_client: Client) {
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

    // async fn create_stream(&self, stream: NewStream) -> Result<String, Box<dyn std::error::Error>> {
    //     // let endpoint: String = format!(
    //     //     "{}{}{}/streams",
    //     //     self.config.ceramic_url, self.config.ceramic_url, self.config.api_version
    //     // );
    //     // // let client: Client = self.config.client.clone();
    //     // let req: RequestBuilder = self.config.client.post(endpoint);
    //     // let res: String = req.json(&stream).send().await?.text().await?;
    //     // stream.stream_type.stream_type = 0;
    //     // return Result::Ok(res);
    //     Ok((String::from("good")))
    // }
}
