// Generated macro for HexUint (trait)
macro_rules! Depcrate_asciiHexUint {
() => {
// Module: crate::ascii
// Provides: {"HexUint"}
// Dependencies: {}
# [doc = " Metadata for parsing hex numbers, see [`hex_uint`]"] pub trait HexUint : Default + Shl < Self , Output = Self > + Add < Self , Output = Self > + From < u8 > { # [doc (hidden)] fn max_nibbles (_ : sealed :: SealedMarker) -> usize ; }
};
}
