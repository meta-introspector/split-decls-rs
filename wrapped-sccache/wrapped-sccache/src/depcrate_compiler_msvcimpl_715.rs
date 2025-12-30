// Generated macro for impl_715 (impl)
macro_rules! Depcrate_compiler_msvcimpl_715 {
() => {
// Module: crate::compiler::msvc
// Provides: {"impl_715"}
// Dependencies: {}
impl SplitMsvcResponseFileArgs < '_ > { # [doc = " Appends backslashes to `target` by decrementing `count`."] # [doc = " If `step` is >1, then `count` is decremented by `step`, resulting in 1 backslash appended for every `step`."] fn append_backslashes_to (target : & mut String , count : & mut usize , step : usize) { while * count >= step { target . push ('\\') ; * count -= step ; } } }
};
}
