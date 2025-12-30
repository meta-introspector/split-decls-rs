// Generated macro for impl_58 (impl)
macro_rules! Depcrate_sip128impl_58 {
() => {
// Module: crate::sip128
// Provides: {"impl_58"}
// Dependencies: {}
impl < S : Sip > Hasher < S > { # [inline] pub fn finish128 (& self) -> Hash128 { let mut state = self . state ; let b : u64 = ((self . length as u64 & 0xff) << 56) | self . tail ; state . v3 ^= b ; S :: c_rounds (& mut state) ; state . v0 ^= b ; state . v2 ^= 0xee ; S :: d_rounds (& mut state) ; let h1 = state . v0 ^ state . v1 ^ state . v2 ^ state . v3 ; state . v1 ^= 0xdd ; S :: d_rounds (& mut state) ; let h2 = state . v0 ^ state . v1 ^ state . v2 ^ state . v3 ; Hash128 { h1 , h2 } } }
};
}
