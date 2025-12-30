// Generated macro for __reject_custom_compilefail (function)
macro_rules! Depcrate_reject__reject_custom_compilefail {
() => {
// Module: crate::reject
// Provides: {"__reject_custom_compilefail"}
// Dependencies: {}
# [doc = " Protect against re-rejecting a rejection."] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " fn with(r: warp::Rejection) {"] # [doc = "     let _wat = warp::reject::custom(r);"] # [doc = " }"] # [doc = " ```"] fn __reject_custom_compilefail () { }
};
}
