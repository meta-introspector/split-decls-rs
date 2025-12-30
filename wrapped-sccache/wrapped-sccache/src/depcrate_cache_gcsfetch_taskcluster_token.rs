// Generated macro for fetch_taskcluster_token (function)
macro_rules! Depcrate_cache_gcsfetch_taskcluster_token {
() => {
// Module: crate::cache::gcs
// Provides: {"fetch_taskcluster_token"}
// Dependencies: {}
# [doc = " Fetch token from TaskCluster for GCS authentication"] # [doc = ""] # [doc = " This feature is required to run [mozilla's CI](https://searchfox.org/mozilla-central/source/build/mozconfig.cache#67-84):"] # [doc = ""] # [doc = " ```txt"] # [doc = " export SCCACHE_GCS_CREDENTIALS_URL=http://taskcluster/auth/v1/gcp/credentials/$SCCACHE_GCS_PROJECT/${bucket}@$SCCACHE_GCS_PROJECT.iam.gserviceaccount.com\""] # [doc = " ```"] # [doc = ""] # [doc = " Reference: [gcpCredentials](https://docs.taskcluster.net/docs/reference/platform/auth/api#gcpCredentials)"] async fn fetch_taskcluster_token (url : & str , scope : & str) -> Result < String > { debug ! ("gcs: start to load token from: {}" , url) ; let user_agent = format ! ("{}/{}" , env ! ("CARGO_PKG_NAME") , env ! ("CARGO_PKG_VERSION")) ; let client = Client :: builder () . user_agent (user_agent) . build () ? ; let res = client . get (url) . send () . await ? ; if res . status () . is_success () { let resp = res . json :: < TaskClusterToken > () . await ? ; debug ! ("gcs: token load succeeded for scope: {}" , scope) ; Ok (resp . access_token) } else { let status_code = res . status () ; let content = res . text () . await ? ; Err (anyhow ! ("token load failed for: code: {status_code}, {content}")) } }
};
}
