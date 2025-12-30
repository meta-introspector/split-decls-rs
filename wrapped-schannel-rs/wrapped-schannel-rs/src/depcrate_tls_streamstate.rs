// Generated macro for State (enum)
macro_rules! Depcrate_tls_streamState {
() => {
// Module: crate::tls_stream
// Provides: {"State"}
// Dependencies: {}
enum State { Initializing { needs_flush : bool , more_calls : bool , shutting_down : bool , validated : bool , } , Streaming { sizes : Identity :: SecPkgContext_StreamSizes , } , Shutdown , }
};
}
