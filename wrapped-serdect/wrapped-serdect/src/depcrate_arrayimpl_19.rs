// Generated macro for impl_19 (impl)
macro_rules! Depcrate_arrayimpl_19 {
() => {
// Module: crate::array
// Provides: {"impl_19"}
// Dependencies: {}
impl < const N : usize , const UPPERCASE : bool > From < [u8 ; N] > for HexOrBin < N , UPPERCASE > { fn from (bytes : [u8 ; N]) -> Self { Self (bytes) } }
};
}
