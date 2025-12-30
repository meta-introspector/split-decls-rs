// Generated macro for pathvec (macro)
macro_rules! Depcrate_test_utilspathvec {
() => {
// Module: crate::test::utils
// Provides: {"pathvec"}
// Dependencies: {}
# [doc = " Return a `Vec` with each listed entry converted to an owned `PathBuf`."] macro_rules ! pathvec { ($ ($ x : expr) ,*) => { vec ! ($ (:: std :: path :: PathBuf :: from ($ x) ,) *) } ; }
};
}
