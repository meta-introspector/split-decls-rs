// Generated macro for json_response (function)
macro_rules! Depcrate_dist_client_authjson_response {
() => {
// Module: crate::dist::client_auth
// Provides: {"json_response"}
// Dependencies: {}
fn json_response < T : Serialize > (data : & T) -> Result < Response < Full < Bytes > > > { let body = serde_json :: to_vec (data) . context ("Failed to serialize to JSON") ? ; let len = body . len () ; Ok (Response :: builder () . header (CONTENT_TYPE , mime :: APPLICATION_JSON . to_string ()) . header (CONTENT_LENGTH , len) . body (body . into ()) . unwrap ()) }
};
}
