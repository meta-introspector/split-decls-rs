// Generated macro for impl_248 (impl)
macro_rules! Depcrate_privateimpl_248 {
() => {
// Module: crate::private
// Provides: {"impl_248"}
// Dependencies: {}
impl < IU : InvertedUnsigned , B : Bit > InvertedUnsigned for InvertedUInt < IU , B > { # [inline] fn to_u64 () -> u64 { u64 :: from (B :: to_u8 ()) | IU :: to_u64 () << 1 } }
};
}
