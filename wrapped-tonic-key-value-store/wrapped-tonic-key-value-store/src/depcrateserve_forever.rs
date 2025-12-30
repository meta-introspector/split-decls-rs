// Generated macro for serve_forever (function)
macro_rules! Depcrateserve_forever {
() => {
// Module: crate
// Provides: {"serve_forever"}
// Dependencies: {}
async fn serve_forever (listener : TcpListener) -> Result < () , Box < dyn std :: error :: Error > > { let db = Arc :: new (RwLock :: new (HashMap :: new ())) ; let (tx , rx) = broadcast :: channel (1024) ; drop (rx) ; let service = key_value_store_server :: KeyValueStoreServer :: new (ServerImpl { db , tx }) ; let classifier = GrpcErrorsAsFailures :: new () . with_success (GrpcCode :: InvalidArgument) . with_success (GrpcCode :: NotFound) ; let layer = ServiceBuilder :: new () . timeout (Duration :: from_secs (10)) . layer (CompressionLayer :: new ()) . layer (SetSensitiveHeadersLayer :: new (once (header :: AUTHORIZATION))) . layer (TraceLayer :: new (SharedClassifier :: new (classifier)) . make_span_with (DefaultMakeSpan :: new () . include_headers (true)) ,) . into_inner () ; let addr = listener . local_addr () ? ; tracing :: info ! ("Listening on {}" , addr) ; tonic :: transport :: Server :: builder () . layer (layer) . add_service (service) . serve_with_incoming (TcpListenerStream :: new (listener)) . await ? ; Ok (()) }
};
}
