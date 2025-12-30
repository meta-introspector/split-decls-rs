// Generated macro for const_unreachable (macro)
macro_rules! Depcrate_util_macrosconst_unreachable {
() => {
// Module: crate::util::macros
// Provides: {"const_unreachable"}
// Dependencies: {}
# [doc = " Either invoke `unreachable!()` or `loop {}` depending on whether the Rust"] # [doc = " toolchain supports panicking in `const fn`."] macro_rules ! const_unreachable { () => { { # [cfg (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)] unreachable ! () ; # [cfg (not (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0))] loop { } } } ; }
};
}
