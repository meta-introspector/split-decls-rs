// Generated macro for impl_766 (impl)
macro_rules! Depcrate_replyimpl_766 {
() => {
// Module: crate::reply
// Provides: {"impl_766"}
// Dependencies: {}
impl < T , E > Reply for Result < T , E > where T : Reply , E : Reply , { # [inline] fn into_response (self) -> Response { match self { Ok (t) => t . into_response () , Err (e) => e . into_response () , } } }
};
}
