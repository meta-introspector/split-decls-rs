// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl InlineSize { # [doc = " SAFETY: `value` must be less than or equal to [`INLINE_CAP`]"] # [inline (always)] const unsafe fn transmute_from_u8 (value : u8) -> Self { debug_assert ! (value <= InlineSize :: _V23 as u8) ; unsafe { mem :: transmute :: < u8 , Self > (value) } } }
};
}
