// Generated macro for respond (function)
macro_rules! Depcraterespond {
() => {
// Module: crate
// Provides: {"respond"}
// Dependencies: {}
# [doc = " Convert a GraphQL response to a Tide response."] pub fn respond (resp : impl Into < async_graphql :: BatchResponse >) -> tide :: Result { let resp = resp . into () ; let mut response = Response :: new (StatusCode :: Ok) ; if resp . is_ok () { if let Some (cache_control) = resp . cache_control () . value () { response . insert_header (headers :: CACHE_CONTROL , cache_control) ; } } for (name , value) in resp . http_headers_iter () { if let Ok (value) = value . to_str () { response . append_header (name . as_str () , value) ; } } response . set_body (Body :: from_json (& resp) ?) ; Ok (response) }
};
}
