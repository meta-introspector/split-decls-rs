// Generated macro for impl_189 (impl)
macro_rules! Depcrate_cache_gcsimpl_189 {
() => {
// Module: crate::cache::gcs
// Provides: {"impl_189"}
// Dependencies: {}
impl GCSCache { # [doc = " Create a new `GCSCache` storing data in `bucket`"] pub fn build (bucket : & str , key_prefix : & str , cred_path : Option < & str > , service_account : Option < & str > , rw_mode : CacheMode , credential_url : Option < & str > ,) -> Result < Operator > { let mut builder = Gcs :: default () . bucket (bucket) . root (key_prefix) . scope (rw_to_scope (rw_mode)) ; if let Some (service_account) = service_account { builder = builder . service_account (service_account) ; } if let Some (path) = cred_path { builder = builder . credential_path (path) ; } if let Some (cred_url) = credential_url { let _ = Url :: parse (cred_url) . map_err (| err | anyhow ! ("gcs credential url is invalid: {err:?}")) ? ; let token = tokio :: task :: block_in_place (| | { tokio :: runtime :: Handle :: current () . block_on (fetch_taskcluster_token (cred_url , rw_to_scope (rw_mode))) }) . map_err (| e | anyhow ! ("Failed to fetch TaskCluster token: {e}")) ? ; builder = builder . token (token) ; } let op = Operator :: new (builder) ? . layer (HttpClientLayer :: new (set_user_agent ())) . layer (LoggingLayer :: default ()) . finish () ; Ok (op) } }
};
}
