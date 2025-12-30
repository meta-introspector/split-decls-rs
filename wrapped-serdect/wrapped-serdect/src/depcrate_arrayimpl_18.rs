// Generated macro for impl_18 (impl)
macro_rules! Depcrate_arrayimpl_18 {
() => {
// Module: crate::array
// Provides: {"impl_18"}
// Dependencies: {}
impl < const N : usize , const UPPERCASE : bool > From < & [u8 ; N] > for HexOrBin < N , UPPERCASE > { fn from (bytes : & [u8 ; N]) -> Self { Self (* bytes) } }
};
}
