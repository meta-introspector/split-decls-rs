// Generated macro for impl_216 (impl)
macro_rules! Depcrate_cache_ossimpl_216 {
() => {
// Module: crate::cache::oss
// Provides: {"impl_216"}
// Dependencies: {}
impl OSSCache { pub fn build (bucket : & str , key_prefix : & str , endpoint : Option < & str > , no_credentials : bool ,) -> Result < Operator > { let mut builder = Oss :: default () . bucket (bucket) . root (key_prefix) ; if let Some (endpoint) = endpoint { builder = builder . endpoint (endpoint) ; } if no_credentials { builder = builder . allow_anonymous () ; } let op = Operator :: new (builder) ? . layer (HttpClientLayer :: new (set_user_agent ())) . layer (LoggingLayer :: default ()) . finish () ; Ok (op) } }
};
}
