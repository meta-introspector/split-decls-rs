// Generated macro for impl_185 (impl)
macro_rules! Depcrate_tinyvecimpl_185 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_185"}
// Dependencies: {}
impl < A : Array > Debug for TinyVecIterator < A > where A :: Item : Debug , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("TinyVecIterator") . field (& self . as_slice ()) . finish () } }
};
}
