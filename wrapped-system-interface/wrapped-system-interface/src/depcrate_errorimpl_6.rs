// Generated macro for impl_6 (impl)
macro_rules! Depcrate_errorimpl_6 {
() => {
// Module: crate::error
// Provides: {"impl_6"}
// Dependencies: {}
impl FromPrimitive for SystemError { # [inline] fn from_i64 (n : i64) -> Option < Self > { if n == Self :: AccountAlreadyInUse as i64 { Some (Self :: AccountAlreadyInUse) } else if n == Self :: ResultWithNegativeLamports as i64 { Some (Self :: ResultWithNegativeLamports) } else if n == Self :: InvalidProgramId as i64 { Some (Self :: InvalidProgramId) } else if n == Self :: InvalidAccountDataLength as i64 { Some (Self :: InvalidAccountDataLength) } else if n == Self :: MaxSeedLengthExceeded as i64 { Some (Self :: MaxSeedLengthExceeded) } else if n == Self :: AddressWithSeedMismatch as i64 { Some (Self :: AddressWithSeedMismatch) } else if n == Self :: NonceNoRecentBlockhashes as i64 { Some (Self :: NonceNoRecentBlockhashes) } else if n == Self :: NonceBlockhashNotExpired as i64 { Some (Self :: NonceBlockhashNotExpired) } else if n == Self :: NonceUnexpectedBlockhashValue as i64 { Some (Self :: NonceUnexpectedBlockhashValue) } else { None } } # [inline] fn from_u64 (n : u64) -> Option < Self > { Self :: from_i64 (n as i64) } }
};
}
