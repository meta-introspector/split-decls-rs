// Generated macro for to_le_uint (function)
macro_rules! Depcrate_binaryto_le_uint {
() => {
// Module: crate::binary
// Provides: {"to_le_uint"}
// Dependencies: {}
# [inline] fn to_le_uint < Input , Uint > (number : & Input , offset : usize) -> Uint where Input : Stream , Uint : Default + Shl < u8 , Output = Uint > + Add < Uint , Output = Uint > + From < < Input as Stream > :: Token > , { let mut res = Uint :: default () ; for (index , byte) in number . iter_offsets () . take (offset) { res = res + (Uint :: from (byte) << (8 * index as u8)) ; } res }
};
}
