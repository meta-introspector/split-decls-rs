// Generated macro for set_user_agent (function)
macro_rules! Depcrate_cache_http_clientset_user_agent {
() => {
// Module: crate::cache::http_client
// Provides: {"set_user_agent"}
// Dependencies: {}
# [doc = " Set the user agent (helps with monitoring on the server side)"] pub fn set_user_agent () -> HttpClient { let user_agent = format ! ("{}/{}" , env ! ("CARGO_PKG_NAME") , env ! ("CARGO_PKG_VERSION")) ; let client = ClientBuilder :: new () . user_agent (user_agent) . build () . unwrap () ; HttpClient :: with (client) }
};
}
