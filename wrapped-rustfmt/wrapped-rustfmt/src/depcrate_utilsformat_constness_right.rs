// Generated macro for format_constness_right (function)
macro_rules! Depcrate_utilsformat_constness_right {
() => {
// Module: crate::utils
// Provides: {"format_constness_right"}
// Dependencies: {}
# [inline] pub (crate) fn format_constness_right (constness : ast :: Const) -> & 'static str { match constness { ast :: Const :: Yes (..) => " const" , ast :: Const :: No => "" , } }
};
}
