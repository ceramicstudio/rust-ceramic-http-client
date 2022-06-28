use async_trait::async_trait;

use crate::stream::genesis_commit::GenesisCommit;

#[async_trait]
pub trait CeramicHTTPRequests {
    async fn get_supported_chains(&self) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_stream(&self, stream_id: &str) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_all_stream_commits(
        &self,
        stream_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_pin(&self, stream_id: &str) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_healthcheck(&self) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_stream_from_genesis(
        &self,
        genesis: GenesisCommit,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
