// Generated macro for impl_639 (impl)
macro_rules! Depcrate_io_uringimpl_639 {
() => {
// Module: crate::io_uring
// Provides: {"impl_639"}
// Dependencies: {}
# [allow (clippy :: non_canonical_partial_ord_impl)] impl PartialOrd for io_uring_user_data { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { unsafe { self . u64_ . partial_cmp (& other . u64_) } } }
};
}
