// Generated macro for ovec (macro)
macro_rules! Depcrate_test_utilsovec {
() => {
// Module: crate::test::utils
// Provides: {"ovec"}
// Dependencies: {}
# [doc = " Return a `Vec` with each listed entry converted to an owned `OsString`."] macro_rules ! ovec { ($ ($ x : expr) ,*) => { vec ! ($ (:: std :: ffi :: OsString :: from ($ x) ,) *) } ; }
};
}
