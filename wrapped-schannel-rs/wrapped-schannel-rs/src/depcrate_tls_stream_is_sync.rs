// Generated macro for _is_sync (function)
macro_rules! Depcrate_tls_stream_is_sync {
() => {
// Module: crate::tls_stream
// Provides: {"_is_sync"}
// Dependencies: {}
# [doc = " ensures that a TlsStream is always Sync/Send"] fn _is_sync () { fn sync < T : Sync + Send > () { } sync :: < TlsStream < () > > () ; }
};
}
