// Generated macro for impl_45 (impl)
macro_rules! Depcrate_requestimpl_45 {
() => {
// Module: crate::request
// Provides: {"impl_45"}
// Dependencies: {}
impl From < http :: Request > for Request { # [doc = " Converts an `http::Request` to a `surf::Request`."] fn from (req : http :: Request) -> Self { Self { req , middleware : None , } } }
};
}
