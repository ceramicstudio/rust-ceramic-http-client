use crate::ceramic_client::ceramic_config::CeramicConfig;
use crate::ceramic_client::http_requests::CeramicHTTPRequests;
use async_trait::async_trait;
use reqwest::RequestBuilder;

pub struct CeramicHTTPClient {
    pub config: CeramicConfig,
}

impl CeramicHTTPClient {
    pub fn new(config: Option<CeramicConfig>) -> CeramicHTTPClient {
        Self {
            config: config.unwrap_or_default(),
        }
    }
}

#[async_trait]
impl CeramicHTTPRequests for CeramicHTTPClient {
    async fn get_stream(&self, stream_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String = format!(
            "{}{}/streams/{}",
            self.config.ceramic_url, self.config.api_version, stream_id
        );
        let req: RequestBuilder = self.config.client.get(endpoint);
        let res: String = req.send().await?.text().await?;
        return Result::Ok(res);
    }

    async fn get_supported_chains(&self) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String = format!(
            "{}{}{}/node/chains",
            self.config.ceramic_url, self.config.ceramic_url, self.config.api_version
        );
        let req: RequestBuilder = self.config.client.get(endpoint);
        let res: String = req.send().await?.text().await?;
        return Result::Ok(res);
    }

    async fn get_all_stream_commits(
        &self,
        stream_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String = format!(
            "{}{}/commits/{}",
            self.config.ceramic_url, self.config.api_version, stream_id
        );
        let req: RequestBuilder = self.config.client.get(endpoint);
        let res: String = req.send().await?.text().await?;
        return Result::Ok(res);
    }

    async fn get_pin(&self, stream_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String = format!(
            "{}{}/pins/{}",
            self.config.ceramic_url, self.config.api_version, stream_id
        );
        let req: RequestBuilder = self.config.client.get(endpoint);
        let res: String = req.send().await?.text().await?;
        return Result::Ok(res);
    }

    async fn get_healthcheck(&self) -> Result<String, Box<dyn std::error::Error>> {
        let endpoint: String = format!(
            "{}{}/node/healthcheck",
            self.config.ceramic_url, self.config.api_version
        );
        let req: RequestBuilder = self.config.client.get(endpoint);
        let res: String = req.send().await?.text().await?;
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
