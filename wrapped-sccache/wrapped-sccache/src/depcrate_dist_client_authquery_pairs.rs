// Generated macro for query_pairs (function)
macro_rules! Depcrate_dist_client_authquery_pairs {
() => {
// Module: crate::dist::client_auth
// Provides: {"query_pairs"}
// Dependencies: {}
fn query_pairs (url : & str) -> Result < HashMap < String , String > > { let url = Url :: parse ("http://unused_base") . expect ("Failed to parse fake url prefix") . join (url) . context ("Failed to parse url while extracting query params") ? ; Ok (url . query_pairs () . map (| (k , v) | (k . into_owned () , v . into_owned ())) . collect ()) }
};
}
