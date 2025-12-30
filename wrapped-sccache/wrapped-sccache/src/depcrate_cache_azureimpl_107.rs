// Generated macro for impl_107 (impl)
macro_rules! Depcrate_cache_azureimpl_107 {
() => {
// Module: crate::cache::azure
// Provides: {"impl_107"}
// Dependencies: {}
impl AzureBlobCache { pub fn build (connection_string : & str , container : & str , key_prefix : & str) -> Result < Operator > { let builder = Azblob :: from_connection_string (connection_string) ? . container (container) . root (key_prefix) ; let op = Operator :: new (builder) ? . layer (HttpClientLayer :: new (set_user_agent ())) . layer (LoggingLayer :: default ()) . finish () ; Ok (op) } }
};
}
