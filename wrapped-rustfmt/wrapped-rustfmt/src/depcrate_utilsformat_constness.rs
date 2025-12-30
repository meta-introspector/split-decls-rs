// Generated macro for format_constness (function)
macro_rules! Depcrate_utilsformat_constness {
() => {
// Module: crate::utils
// Provides: {"format_constness"}
// Dependencies: {}
# [inline] pub (crate) fn format_constness (constness : ast :: Const) -> & 'static str { match constness { ast :: Const :: Yes (..) => "const " , ast :: Const :: No => "" , } }
};
}
