// Generated macro for impl_574 (impl)
macro_rules! Depcrate_ule_charsimpl_574 {
() => {
// Module: crate::ule::chars
// Provides: {"impl_574"}
// Dependencies: {}
impl CharULE { # [doc = " Converts a [`char`] to a [`CharULE`]. This is equivalent to calling"] # [doc = " [`AsULE::to_unaligned()`]"] # [doc = ""] # [doc = " See the type-level documentation for [`CharULE`] for more information."] # [inline] pub const fn from_aligned (c : char) -> Self { let [u0 , u1 , u2 , _u3] = (c as u32) . to_le_bytes () ; Self ([u0 , u1 , u2]) } # [doc = " Converts this [`CharULE`] to a [`char`]. This is equivalent to calling"] # [doc = " [`AsULE::from_unaligned`]"] # [doc = ""] # [doc = " See the type-level documentation for [`CharULE`] for more information."] # [inline] pub fn to_char (self) -> char { let [b0 , b1 , b2] = self . 0 ; unsafe { char :: from_u32_unchecked (u32 :: from_le_bytes ([b0 , b1 , b2 , 0])) } } impl_ule_from_array ! (char , CharULE , Self ([0 ; 3])) ; }
};
}
