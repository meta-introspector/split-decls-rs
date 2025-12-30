// Generated macro for stringvec (macro)
macro_rules! Depcrate_test_utilsstringvec {
() => {
// Module: crate::test::utils
// Provides: {"stringvec"}
// Dependencies: {}
# [doc = " Return a `Vec` with each listed entry converted to an owned `String`."] macro_rules ! stringvec { ($ ($ x : expr) ,*) => { vec ! ($ ($ x . to_owned () ,) *) } ; }
};
}
