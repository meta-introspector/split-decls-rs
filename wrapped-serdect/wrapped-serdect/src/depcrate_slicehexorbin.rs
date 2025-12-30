// Generated macro for HexOrBin (struct)
macro_rules! Depcrate_sliceHexOrBin {
() => {
// Module: crate::slice
// Provides: {"HexOrBin"}
// Dependencies: {}
# [doc = " Serializer/deserializer newtype which encodes bytes as either binary or hex."] # [doc = ""] # [doc = " Use hexadecimal with human-readable formats, or raw binary with binary formats."] # [cfg (feature = "alloc")] # [derive (Clone , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct HexOrBin < const UPPERCASE : bool > (pub Vec < u8 >) ;
};
}
