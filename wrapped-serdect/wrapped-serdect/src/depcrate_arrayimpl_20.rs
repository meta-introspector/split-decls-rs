// Generated macro for impl_20 (impl)
macro_rules! Depcrate_arrayimpl_20 {
() => {
// Module: crate::array
// Provides: {"impl_20"}
// Dependencies: {}
impl < const N : usize , const UPPERCASE : bool > From < HexOrBin < N , UPPERCASE > > for [u8 ; N] { fn from (hex_or_bin : HexOrBin < N , UPPERCASE >) -> Self { hex_or_bin . 0 } }
};
}
