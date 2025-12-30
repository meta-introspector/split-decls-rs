// Generated macro for impl_7 (impl)
macro_rules! Depcrate_errorimpl_7 {
() => {
// Module: crate::error
// Provides: {"impl_7"}
// Dependencies: {}
impl ToPrimitive for SystemError { # [inline] fn to_i64 (& self) -> Option < i64 > { Some (match * self { Self :: AccountAlreadyInUse => Self :: AccountAlreadyInUse as i64 , Self :: ResultWithNegativeLamports => Self :: ResultWithNegativeLamports as i64 , Self :: InvalidProgramId => Self :: InvalidProgramId as i64 , Self :: InvalidAccountDataLength => Self :: InvalidAccountDataLength as i64 , Self :: MaxSeedLengthExceeded => Self :: MaxSeedLengthExceeded as i64 , Self :: AddressWithSeedMismatch => Self :: AddressWithSeedMismatch as i64 , Self :: NonceNoRecentBlockhashes => Self :: NonceNoRecentBlockhashes as i64 , Self :: NonceBlockhashNotExpired => Self :: NonceBlockhashNotExpired as i64 , Self :: NonceUnexpectedBlockhashValue => Self :: NonceUnexpectedBlockhashValue as i64 , }) } # [inline] fn to_u64 (& self) -> Option < u64 > { self . to_i64 () . map (| x | x as u64) } }
};
}
