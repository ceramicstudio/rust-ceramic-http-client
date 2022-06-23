pub mod ceramic_client {
    pub mod ceramic_http_client;
    pub mod http_requests;
}

#[cfg(test)]
mod tests {
    use crate::ceramic_client::ceramic_http_client::CeramicHTTPClient;

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
        ceramic.set_did("did:3:0x1234567890123456789012345678901234567890");
        assert_eq!(
            ceramic.get_did(),
            Some("did:3:0x1234567890123456789012345678901234567890".to_string())
        );
    }

    #[test]
    fn ceramic_client_sets_and_gets_api_url() {
        let mut ceramic: CeramicHTTPClient = CeramicHTTPClient::default();
        ceramic.set_api_url("https://new/api");
        let api_url = ceramic.get_api_url();
        assert_eq!(api_url, "https://new/api");
    }

    #[test]
    fn ceramic_client_sets_and_gets_api_version() {
        let mut ceramic: CeramicHTTPClient = CeramicHTTPClient::default();
        ceramic.set_api_version("v1");
        let api_version = ceramic.get_api_version();
        assert_eq!(api_version, "v1");
    }

    #[test]
    fn ceramic_client_sets_and_gets_ceramic_network_url() {
        let mut ceramic: CeramicHTTPClient = CeramicHTTPClient::default();
        ceramic.set_ceramic_network_url("https://new-url");
        let ceramic_network_url = ceramic.get_ceramic_network_url();
        assert_eq!(ceramic_network_url, "https://new-url");
    }
}
