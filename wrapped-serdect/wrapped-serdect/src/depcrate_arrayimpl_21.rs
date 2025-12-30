// Generated macro for impl_21 (impl)
macro_rules! Depcrate_arrayimpl_21 {
() => {
// Module: crate::array
// Provides: {"impl_21"}
// Dependencies: {}
impl < const N : usize , const UPPERCASE : bool > Serialize for HexOrBin < N , UPPERCASE > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if UPPERCASE { serialize_hex_upper_or_bin (self , serializer) } else { serialize_hex_lower_or_bin (self , serializer) } } }
};
}
