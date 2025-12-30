// Generated macro for ensure_usable_cors_rules (function)
macro_rules! Depcrate_corsensure_usable_cors_rules {
() => {
// Module: crate::cors
// Provides: {"ensure_usable_cors_rules"}
// Dependencies: {}
fn ensure_usable_cors_rules (layer : & CorsLayer) { if layer . allow_credentials . is_true () { assert ! (! layer . allow_headers . is_wildcard () , "Invalid CORS configuration: Cannot combine `Access-Control-Allow-Credentials: true` \
             with `Access-Control-Allow-Headers: *`") ; assert ! (! layer . allow_methods . is_wildcard () , "Invalid CORS configuration: Cannot combine `Access-Control-Allow-Credentials: true` \
             with `Access-Control-Allow-Methods: *`") ; assert ! (! layer . allow_origin . is_wildcard () , "Invalid CORS configuration: Cannot combine `Access-Control-Allow-Credentials: true` \
             with `Access-Control-Allow-Origin: *`") ; assert ! (! layer . expose_headers . is_wildcard () , "Invalid CORS configuration: Cannot combine `Access-Control-Allow-Credentials: true` \
             with `Access-Control-Expose-Headers: *`") ; } }
};
}
