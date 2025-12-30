// Generated macro for impl_462 (impl)
macro_rules! Depcrate_wrappersimpl_462 {
() => {
// Module: crate::wrappers
// Provides: {"impl_462"}
// Dependencies: {}
impl < T : Unaligned + Hash > Hash for Unalign < T > { # [inline (always)] fn hash < H > (& self , state : & mut H) where H : Hasher , { self . deref () . hash (state) ; } }
};
}
