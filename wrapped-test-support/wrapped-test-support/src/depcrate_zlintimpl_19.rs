// Generated macro for impl_19 (impl)
macro_rules! Depcrate_zlintimpl_19 {
() => {
// Module: crate::zlint
// Provides: {"impl_19"}
// Dependencies: {}
impl LintResult { pub fn check_lints (& self , ignored : & [& str]) -> bool { let mut failed = HashMap :: < String , LintStatus > :: new () ; for (key , value) in & self . 0 { if ! value . is_successful () && ! ignored . contains (& key . as_str ()) { failed . insert (String :: from (key) , value . clone ()) ; } } eprintln ! ("failed lints: {failed:?}") ; failed . is_empty () } }
};
}
