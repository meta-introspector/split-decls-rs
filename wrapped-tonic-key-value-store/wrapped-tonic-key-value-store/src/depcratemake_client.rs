// Generated macro for make_client (function)
macro_rules! Depcratemake_client {
() => {
// Module: crate
// Provides: {"make_client"}
// Dependencies: {}
async fn make_client (addr : SocketAddr ,) -> Result < KeyValueStoreClient < impl Service < http :: Request < Body > , Response = http :: Response < impl HttpBody < Data = Bytes , Error = impl Into < BoxError > > > , Error = impl Into < BoxError > , > + Clone + Send + Sync + 'static , > , tonic :: transport :: Error , > { let uri = format ! ("http://{}" , addr) . parse :: < tonic :: transport :: Uri > () . unwrap () ; let channel = Channel :: builder (uri) . connect () . await ? ; let channel = ServiceBuilder :: new () . layer (DecompressionLayer :: new ()) . layer (SetRequestHeaderLayer :: overriding (header :: USER_AGENT , HeaderValue :: from_static ("tonic-key-value-store") ,)) . layer (TraceLayer :: new_for_grpc () . make_span_with (DefaultMakeSpan :: new () . include_headers (true)) ,) . service (channel) ; Ok (KeyValueStoreClient :: new (channel)) }
};
}
