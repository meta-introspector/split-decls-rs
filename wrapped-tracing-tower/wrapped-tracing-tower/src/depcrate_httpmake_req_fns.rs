// Generated macro for make_req_fns (macro)
macro_rules! Depcrate_httpmake_req_fns {
() => {
// Module: crate::http
// Provides: {"make_req_fns"}
// Dependencies: {}
macro_rules ! make_req_fns { ($ ($ name : ident , $ level : expr) ,+) => { $ (# [inline] pub fn $ name < A > (req : & http :: Request < A >) -> tracing :: Span { tracing :: span ! ($ level , "request" , method = ? req . method () , uri = ? req . uri () ,) }) + } }
};
}
