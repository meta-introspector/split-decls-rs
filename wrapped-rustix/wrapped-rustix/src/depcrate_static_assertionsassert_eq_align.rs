// Generated macro for assert_eq_align (macro)
macro_rules! Depcrate_static_assertionsassert_eq_align {
() => {
// Module: crate::static_assertions
// Provides: {"assert_eq_align"}
// Dependencies: {}
macro_rules ! assert_eq_align { ($ x : ty , $ y : ty) => { assert_eq ! (core :: mem :: align_of ::<$ x > () , core :: mem :: align_of ::<$ y > ()) ; } ; }
};
}
