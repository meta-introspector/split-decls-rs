// Generated macro for endpoint_resolver (function)
macro_rules! Depcrate_cache_s3endpoint_resolver {
() => {
// Module: crate::cache::s3
// Provides: {"endpoint_resolver"}
// Dependencies: {}
# [doc = " Resolve given endpoint along with use_ssl settings."] fn endpoint_resolver (endpoint : & str , use_ssl : Option < bool >) -> Result < String > { let endpoint_uri : http :: Uri = endpoint . try_into () . map_err (| err | anyhow ! ("input endpoint {endpoint} is invalid: {:?}" , err)) ? ; let mut parts = endpoint_uri . into_parts () ; match use_ssl { Some (true) => { parts . scheme = Some (http :: uri :: Scheme :: HTTPS) ; } Some (false) => { parts . scheme = Some (http :: uri :: Scheme :: HTTP) ; } None => { if parts . scheme . is_none () { parts . scheme = Some (http :: uri :: Scheme :: HTTP) ; } } } if parts . path_and_query . is_none () { parts . path_and_query = Some (http :: uri :: PathAndQuery :: from_static ("/")) ; } Ok (http :: Uri :: from_parts (parts) ? . to_string ()) }
};
}
