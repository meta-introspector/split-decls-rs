// Generated macro for build_array_result (function)
macro_rules! Depcrate_buildbuild_array_result {
() => {
// Module: crate::build
// Provides: {"build_array_result"}
// Dependencies: {}
# [doc = " Build an array with a function that creates elements based on their value, short-circuiting if"] # [doc = " any index returns an `Err`"] # [doc = ""] # [doc = " ```"] # [doc = " # use unarray::*;"] # [doc = ""] # [doc = " let success: Result<_, ()> = build_array_result(|i| Ok(i * 2));"] # [doc = " assert_eq!(success, Ok([0, 2, 4]));"] # [doc = " ```"] # [doc = ""] # [doc = " If `f` panics, any already-initialized elements will be dropped **without** running their"] # [doc = " `Drop` implmentations, potentially creating resource leaks. Note that this is still \"safe\","] # [doc = " since Rust's notion of \"safety\" doesn't guarantee destructors are run."] # [doc = ""] # [doc = " This is similar to the nightly-only [`core::array::try_from_fn`]"] pub fn build_array_result < T , E , F : FnMut (usize) -> Result < T , E > , const N : usize > (mut f : F ,) -> Result < [T ; N] , E > { let mut result = uninit_buf () ; for (index , slot) in result . iter_mut () . enumerate () { match f (index) { Ok (value) => slot . write (value) , Err (e) => { result . iter_mut () . take (index) . for_each (| slot | unsafe { slot . assume_init_drop () }) ; return Err (e) ; } } ; } Ok (unsafe { mark_initialized (result) }) }
};
}
