// Generated macro for client_tests (module)
macro_rules! Depcrate_clientclient_tests {
() => {
// Module: crate::client
// Provides: {"client_tests"}
// Dependencies: {}
# [cfg (test)] mod client_tests { use std :: convert :: TryInto ; use super :: Client ; use super :: Config ; use crate :: Url ; # [test] fn base_url () { let base_url = Url :: parse ("http://example.com/api/v1/") . unwrap () ; let client : Client = Config :: new () . set_base_url (base_url) . try_into () . unwrap () ; let url = client . url ("posts.json") ; assert_eq ! (url . as_str () , "http://example.com/api/v1/posts.json") ; } }
};
}
