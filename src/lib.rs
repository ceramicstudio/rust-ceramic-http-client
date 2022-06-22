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
        assert_eq!(ceramic.api_url, "https://ceramic-clay.3boxlabs.com/api");
        assert_eq!(ceramic.api_version, "/v0");
        assert_eq!(
            ceramic.ceramic_network_url,
            "https://ceramic-clay.3boxlabs.com"
        );
    }

    #[test]
    fn ceramic_client_uses_new_constructor() {
        let ceramic: CeramicHTTPClient = CeramicHTTPClient::new(
            "https://ceramic-clay.3boxlabs.com/api",
            "v1",
            "https://ceramic-clay.3boxlabs.com",
            Default::default(),
        );
        assert_eq!(ceramic.api_url, "https://ceramic-clay.3boxlabs.com/api");
        assert_eq!(ceramic.api_version, "v1");
        assert_eq!(
            ceramic.ceramic_network_url,
            "https://ceramic-clay.3boxlabs.com"
        );
    }
}
