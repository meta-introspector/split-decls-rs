// Generated macro for impl_56 (impl)
macro_rules! Depcrate_arrayvecimpl_56 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_56"}
// Dependencies: {}
impl < A : Array > Debug for ArrayVecIterator < A > where A :: Item : Debug , { # [allow (clippy :: missing_inline_in_public_items)] fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("ArrayVecIterator") . field (& self . as_slice ()) . finish () } }
};
}
