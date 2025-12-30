// Generated macro for macro_694 (macro)
macro_rules! Depcrate_limit_bodymacro_694 {
() => {
// Module: crate::limit::body
// Provides: {"macro_694"}
// Dependencies: {}
pin_project ! { # [project = BodyProj] enum ResponseBodyInner < B > { PayloadTooLarge { # [pin] body : Full < Bytes >, } , Body { # [pin] body : B } } }
};
}
