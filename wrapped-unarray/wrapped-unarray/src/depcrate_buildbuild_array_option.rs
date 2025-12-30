// Generated macro for build_array_option (function)
macro_rules! Depcrate_buildbuild_array_option {
() => {
// Module: crate::build
// Provides: {"build_array_option"}
// Dependencies: {}
# [doc = " Build an array with a function that creates elements based on their value, short-circuiting if"] # [doc = " any index returns a `None`"] # [doc = ""] # [doc = " ```"] # [doc = " # use unarray::*;"] # [doc = " let success = build_array_option(|i| Some(i * 2));"] # [doc = " assert_eq!(success, Some([0, 2, 4]));"] # [doc = " ```"] # [doc = ""] # [doc = " If `f` panics, any already-initialized elements will be dropped **without** running their"] # [doc = " `Drop` implmentations, potentially creating resource leaks. Note that this is still \"safe\","] # [doc = " since Rust's notion of \"safety\" doesn't guarantee destructors are run."] # [doc = ""] # [doc = " This is similar to the nightly-only [`core::array::try_from_fn`]"] pub fn build_array_option < T , F : FnMut (usize) -> Option < T > , const N : usize > (mut f : F ,) -> Option < [T ; N] > { let actual_f = | i : usize | -> Result < T , () > { f (i) . ok_or (()) } ; match build_array_result (actual_f) { Ok (array) => Some (array) , Err (()) => None , } }
};
}
