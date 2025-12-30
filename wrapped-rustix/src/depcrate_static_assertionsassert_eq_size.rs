// Generated macro for assert_eq_size (macro)
macro_rules! Depcrate_static_assertionsassert_eq_size {
() => {
// Module: crate::static_assertions
// Provides: {"assert_eq_size"}
// Dependencies: {}
macro_rules ! assert_eq_size { ($ x : ty , $ y : ty) => { assert_eq ! (core :: mem :: size_of ::<$ x > () , core :: mem :: size_of ::<$ y > ()) ; } ; }
};
}
