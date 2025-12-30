// Generated macro for test_dbg (macro)
macro_rules! Depcrate_macrostest_dbg {
() => {
// Module: crate::macros
// Provides: {"test_dbg"}
// Dependencies: {}
# [cfg (all (test , loom))] macro_rules ! test_dbg { ($ e : expr) => { match $ e { e => { test_println ! ("{} = {:?}" , stringify ! ($ e) , & e) ; e } } } ; }
};
}
