// Generated macro for musterr (function)
macro_rules! Depcrate_testsmusterr {
() => {
// Module: crate::tests
// Provides: {"musterr"}
// Dependencies: {}
fn musterr (s : & str) { let mut p = Parser :: new (s , None , None , false , ParseMode :: Format) ; p . next () ; assert ! (! p . errors . is_empty ()) ; }
};
}
