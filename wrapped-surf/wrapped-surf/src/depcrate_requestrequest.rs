// Generated macro for Request (struct)
macro_rules! Depcrate_requestRequest {
() => {
// Module: crate::request
// Provides: {"Request"}
// Dependencies: {}
# [doc = " An HTTP request, returns a `Response`."] # [derive (Clone)] pub struct Request { # [doc = " Holds the state of the request."] req : http_client :: Request , # [doc = " Holds an optional per-request middleware stack."] middleware : Option < Vec < Arc < dyn Middleware > > > , }
};
}
