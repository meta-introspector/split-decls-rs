// Generated macro for impl_41 (impl)
macro_rules! Depcrate_configimpl_41 {
() => {
// Module: crate::config
// Provides: {"impl_41"}
// Dependencies: {}
impl From < StringList > for Vec < String > { fn from (list : StringList) -> Vec < String > { match list { StringList :: String (s) => s . split_whitespace () . map (| s | s . to_string ()) . collect () , StringList :: List (s) => s , } } }
};
}
