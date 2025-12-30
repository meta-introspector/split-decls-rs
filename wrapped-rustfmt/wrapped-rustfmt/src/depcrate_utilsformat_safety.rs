// Generated macro for format_safety (function)
macro_rules! Depcrate_utilsformat_safety {
() => {
// Module: crate::utils
// Provides: {"format_safety"}
// Dependencies: {}
# [inline] pub (crate) fn format_safety (unsafety : ast :: Safety) -> & 'static str { match unsafety { ast :: Safety :: Unsafe (..) => "unsafe " , ast :: Safety :: Safe (..) => "safe " , ast :: Safety :: Default => "" , } }
};
}
