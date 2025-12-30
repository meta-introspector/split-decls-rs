// Generated macro for error_code_response (function)
macro_rules! Depcrate_dist_client_autherror_code_response {
() => {
// Module: crate::dist::client_auth
// Provides: {"error_code_response"}
// Dependencies: {}
# [allow (clippy :: unnecessary_wraps)] fn error_code_response < E > (uri : hyper :: Uri , e : E) -> hyper :: Result < Response < Full < Bytes > > > where E : std :: fmt :: Debug , { let body = format ! ("{:?}" , e) ; eprintln ! ("sccache: Error during a request to {} on the client auth web server\n{}" , uri , body) ; let len = body . len () ; let builder = Response :: builder () . status (StatusCode :: INTERNAL_SERVER_ERROR) ; let res = builder . header (CONTENT_TYPE , mime :: TEXT_PLAIN . to_string ()) . header (CONTENT_LENGTH , len) . body (body . into ()) . unwrap () ; Ok :: < Response < Full < Bytes > > , hyper :: Error > (res) }
};
}
