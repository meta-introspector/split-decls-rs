// Generated macro for enable_precise_capture (function)
macro_rules! Depcrate_builder_expr_as_placeenable_precise_capture {
() => {
// Module: crate::builder::expr::as_place
// Provides: {"enable_precise_capture"}
// Dependencies: {}
# [doc = " Precise capture is enabled if user is using Rust Edition 2021 or higher."] fn enable_precise_capture (closure_span : Span) -> bool { closure_span . at_least_rust_2021 () }
};
}
