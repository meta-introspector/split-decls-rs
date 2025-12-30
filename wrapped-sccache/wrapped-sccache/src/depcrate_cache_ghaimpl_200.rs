// Generated macro for impl_200 (impl)
macro_rules! Depcrate_cache_ghaimpl_200 {
() => {
// Module: crate::cache::gha
// Provides: {"impl_200"}
// Dependencies: {}
impl GHACache { pub fn build (version : & str) -> Result < Operator > { let mut builder = Ghac :: default () . root ("/sccache") ; builder = if version . is_empty () { builder . version (& format ! ("sccache-v{VERSION}")) } else { builder . version (& format ! ("sccache-v{VERSION}-{version}")) } ; let op = Operator :: new (builder) ? . layer (HttpClientLayer :: new (set_user_agent ())) . layer (LoggingLayer :: default ()) . finish () ; Ok (op) } }
};
}
