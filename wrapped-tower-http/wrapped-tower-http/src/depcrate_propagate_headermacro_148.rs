// Generated macro for macro_148 (macro)
macro_rules! Depcrate_propagate_headermacro_148 {
() => {
// Module: crate::propagate_header
// Provides: {"macro_148"}
// Dependencies: {}
pin_project ! { # [doc = " Response future for [`PropagateHeader`]."] # [derive (Debug)] pub struct ResponseFuture < F > { # [pin] future : F , header_and_value : Option < (HeaderName , HeaderValue) >, } }
};
}
