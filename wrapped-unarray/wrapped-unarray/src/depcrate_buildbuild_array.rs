// Generated macro for build_array (function)
macro_rules! Depcrate_buildbuild_array {
() => {
// Module: crate::build
// Provides: {"build_array"}
// Dependencies: {}
# [doc = " Build an array with a function that creates elements based on their index"] # [doc = ""] # [doc = " ```"] # [doc = " # use unarray::*;"] # [doc = " let array: [usize; 5] = build_array(|i| i * 2);"] # [doc = " assert_eq!(array, [0, 2, 4, 6, 8]);"] # [doc = " ```"] # [doc = " If `f` panics, any already-initialized elements will be dropped **without** running their"] # [doc = " `Drop` implmentations, potentially creating resource leaks. Note that this is still \"safe\","] # [doc = " since Rust's notion of \"safety\" doesn't guarantee destructors are run."] # [doc = ""] # [doc = " For builder functions which might fail, consider using [`build_array_result`] or"] # [doc = " [`build_array_option`]"] pub fn build_array < T , F : FnMut (usize) -> T , const N : usize > (mut f : F) -> [T ; N] { let mut result = uninit_buf () ; for (index , slot) in result . iter_mut () . enumerate () { let value = f (index) ; slot . write (value) ; } unsafe { mark_initialized (result) } }
};
}
