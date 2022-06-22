# rust-ceramic-http-client
A Rust implementation of the HTTP Ceramic Client

Currently a siginificant work in progess.  You probably don't even want to try and use this yet.


**Usage Example**

Using Defaults

```rust
fn main(){
    // Example StreamID from a Ceramic Stream already on the network
    let stream_id = "kjk...";
    // Create a client with defaults and use that to make read-only requests
    let ceramic = CeramicHTTPClient::default();
    // Get the data from a stream using StreamID
    let stream = ceramic.get_stream(stream_id);
    println!("{:?}",stream);
    
}

```
Custom client

```rust
fn main(){
    // Example StreamID from a Ceramic Stream already on the network
    let stream_id = "kjk...";
    // Create a client with defaults and use that to make read-only requests
    let ceramic = CeramicHTTPClient::default();
    // Use setters to configure custom parameters for the client
    ceramic.set_did("did:3:ksaldjfklsajklf");
    ceramic.set_api_version("/v1");
    ceramic.set_ceramic_network_url("https://localhost:7007");
    // Get the data from a stream using StreamID
    let stream = ceramic.get_stream(stream_id);
    println!("{:?}",stream);
    
}

```