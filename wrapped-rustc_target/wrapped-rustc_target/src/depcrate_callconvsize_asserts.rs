// Generated macro for size_asserts (module)
macro_rules! Depcrate_callconvsize_asserts {
() => {
// Module: crate::callconv
// Provides: {"size_asserts"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] mod size_asserts { use rustc_data_structures :: static_assert_size ; use super :: * ; static_assert_size ! (ArgAbi <'_ , usize >, 56) ; static_assert_size ! (FnAbi <'_ , usize >, 80) ; }
};
}
