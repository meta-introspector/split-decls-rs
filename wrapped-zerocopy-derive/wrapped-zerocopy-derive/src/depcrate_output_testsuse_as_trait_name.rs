// Generated macro for use_as_trait_name (macro)
macro_rules! Depcrate_output_testsuse_as_trait_name {
() => {
// Module: crate::output_tests
// Provides: {"use_as_trait_name"}
// Dependencies: {}
macro_rules ! use_as_trait_name { ($ ($ alias : ident => $ derive : ident) ,* $ (,) ?) => { $ (use super ::$ derive as $ alias ;) * } ; }
};
}
