// Generated macro for impl_257 (impl)
macro_rules! Depcrate_privateimpl_257 {
() => {
// Module: crate::private
// Provides: {"impl_257"}
// Dependencies: {}
impl < IU : InvertedUnsigned , B : Bit > InvertedUnsigned for InvertedUInt < IU , B > { # [inline] fn to_u64 () -> u64 { u64 :: from (B :: to_u8 ()) | IU :: to_u64 () << 1 } }
};
}
