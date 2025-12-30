// Generated macro for exact (function)
macro_rules! Depcrate_filters_hostexact {
() => {
// Module: crate::filters::host
// Provides: {"exact"}
// Dependencies: {}
# [doc = " Creates a `Filter` that requires a specific authority (target server's"] # [doc = " host and port) in the request."] # [doc = ""] # [doc = " Authority is specified either in the `Host` header or in the target URI."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let multihost ="] # [doc = "     warp::host::exact(\"foo.com\").map(|| \"you've reached foo.com\")"] # [doc = "     .or(warp::host::exact(\"bar.com\").map(|| \"you've reached bar.com\"));"] # [doc = " ```"] pub fn exact (expected : & str) -> impl Filter < Extract = () , Error = Rejection > + Clone { let expected = Authority :: from_str (expected) . expect ("invalid host/authority") ; optional () . and_then (move | option : Option < Authority > | match option { Some (authority) if authority == expected => future :: ok (()) , _ => future :: err (reject :: not_found ()) , }) . untuple_one () }
};
}
