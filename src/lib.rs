pub mod ceramic_client {
    pub mod ceramic_http_client;
    pub mod http_requests;
}
pub mod stream {
    pub mod genesis_commit;
    pub mod raw_commit;
}

#[cfg(test)]
mod tests {
    use crate::ceramic_client::ceramic_http_client::CeramicHTTPClient;
    use crate::stream::genesis_commit::GenesisCommit;
    use serde_json::Value;

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

    #[test]
    fn genesis_commit_is_default() {
        let genesis_commit: GenesisCommit = GenesisCommit::default();
        assert_eq!(genesis_commit.get_data(), &Value::Null);
    }
}
