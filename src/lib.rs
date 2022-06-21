pub mod ceramic_client {
    pub mod ceramic_config;
    pub mod ceramic_http_client;
    pub mod http_requests;
}

#[cfg(test)]
mod tests {
    use crate::ceramic_client::ceramic_config::CeramicConfig;

    #[test]
    fn it_works() {
        let x = CeramicConfig::new("local", "v0", reqwest::Client::new()).expect("no config");
        println!("{:?}", x);
        assert_eq!(x.ceramic_url, "http://127.0.0.1:7007/api");
    }
}
