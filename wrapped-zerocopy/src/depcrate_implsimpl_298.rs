// Generated macro for impl_298 (impl)
macro_rules! Depcrate_implsimpl_298 {
() => {
// Module: crate::impls
// Provides: {"impl_298"}
// Dependencies: {}
unsafe impl < T : TryFromBytes + ? Sized > TryFromBytes for UnsafeCell < T > { # [allow (clippy :: missing_inline_in_public_items)] fn only_derive_is_allowed_to_implement_this_trait () where Self : Sized , { } # [inline] fn is_bit_valid < A : invariant :: Reference > (candidate : Maybe < '_ , Self , A >) -> bool { let c = candidate . into_exclusive_or_pme () ; T :: is_bit_valid (c . get_mut ()) } }
};
}
