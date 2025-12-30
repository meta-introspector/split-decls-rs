// Generated macro for assert_name_and_value (function)
macro_rules! Depcrate_filters_replyassert_name_and_value {
() => {
// Module: crate::filters::reply
// Provides: {"assert_name_and_value"}
// Dependencies: {}
fn assert_name_and_value < K , V > (name : K , value : V) -> (HeaderName , HeaderValue) where HeaderName : TryFrom < K > , < HeaderName as TryFrom < K > > :: Error : Into < http :: Error > , HeaderValue : TryFrom < V > , < HeaderValue as TryFrom < V > > :: Error : Into < http :: Error > , { let name = < HeaderName as TryFrom < K > > :: try_from (name) . map_err (Into :: into) . unwrap_or_else (| _ | panic ! ("invalid header name")) ; let value = < HeaderValue as TryFrom < V > > :: try_from (value) . map_err (Into :: into) . unwrap_or_else (| _ | panic ! ("invalid header value")) ; (name , value) }
};
}
