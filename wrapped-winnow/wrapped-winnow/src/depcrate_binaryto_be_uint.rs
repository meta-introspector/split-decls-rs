// Generated macro for to_be_uint (function)
macro_rules! Depcrate_binaryto_be_uint {
() => {
// Module: crate::binary
// Provides: {"to_be_uint"}
// Dependencies: {}
# [inline] fn to_be_uint < Input , Uint > (number : & Input , offset : usize) -> Uint where Input : Stream , Uint : Default + Shl < u8 , Output = Uint > + Add < Uint , Output = Uint > + From < < Input as Stream > :: Token > , { let mut res = Uint :: default () ; for (_ , byte) in number . iter_offsets () . take (offset) { res = (res << 8) + byte . into () ; } res }
};
}
