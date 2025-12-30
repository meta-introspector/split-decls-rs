// Generated macro for HexOrBin (struct)
macro_rules! Depcrate_arrayHexOrBin {
() => {
// Module: crate::array
// Provides: {"HexOrBin"}
// Dependencies: {}
# [doc = " Serializer/deserializer newtype which encodes bytes as either binary or hex."] # [doc = ""] # [doc = " Use hexadecimal with human-readable formats, or raw binary with binary formats."] # [derive (Clone , Copy , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct HexOrBin < const N : usize , const UPPERCASE : bool > (pub [u8 ; N]) ;
};
}
