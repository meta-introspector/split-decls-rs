// Generated macro for should_document (function)
macro_rules! Depcrate_featuresshould_document {
() => {
// Module: crate::features
// Provides: {"should_document"}
// Dependencies: {}
fn should_document (var : & str) -> bool { if var . starts_with ("RUSTC_") || var . starts_with ("RUST_") || var . starts_with ("UNSTABLE_RUSTDOC_") { return true ; } ["SDKROOT" , "QNX_TARGET" , "COLORTERM" , "TERM"] . contains (& var) }
};
}
